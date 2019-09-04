//! This file contains the process description

mod tss;
use tss::TSS;

use super::safe_ffi::{c_char, CStringArray};
use super::syscall::clone::CloneFlags;
use super::SysResult;
use sync::{DeadMutex, DeadMutexGuard};

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::slice;
use fallible_collections::FallibleArc;

use elf_loader::{SegmentType, SymbolTable};
use fallible_collections::{try_vec, FallibleBox};
use libc_binding::Errno;

use crate::elf_loader::load_elf;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages, Page, Virt};
use crate::memory::KERNEL_VIRTUAL_PAGE_ALLOCATOR;
use crate::memory::{mmu::Entry, AddressSpace};
use crate::registers::Eflags;
use crate::system::{BaseRegisters, PrivilegeLevel};

extern "C" {
    fn _start_process(kernel_esp: u32) -> !;
}

/// Arguments given to a process
pub struct ProcessArguments {
    argv: CStringArray,
    envp: CStringArray,
}

impl ProcessArguments {
    pub fn new(argv: CStringArray, envp: CStringArray) -> Self {
        Self { argv, envp }
    }
}

const FX_REGION_LEN: usize = 512;

/// FPU/MMX/SSE/AVX Support
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FxRegion {
    keys: [u8; FX_REGION_LEN],
}

/// Default boilerplate for FxRegion
impl Default for FxRegion {
    fn default() -> Self {
        Self {
            keys: [0; FX_REGION_LEN],
        }
    }
}

/// Represent all the cpu states of a process according to the TSS context
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct CpuState {
    /// reserved for back trace
    pub stack_reserved: u32,
    /// FPU/MMX/SSE/AVX Support
    pub fx_region: FxRegion,
    /// Padding of 4 bytes to ensure that fx_region is 16 bytes aligned
    pub fx_padding: u32,
    /// current registers
    pub registers: BaseRegisters,
    /// current data GS
    pub gs: u32,
    /// current data FS
    pub fs: u32,
    /// current data ES
    pub es: u32,
    /// current data DS
    pub ds: u32,
    /// the number of the cpu isr (for cpu exceptions)
    pub cpu_isr_reserved: u32,
    /// error code reserved (for some cpu exceptions)
    pub err_code_reserved: u32,
    /// current eip
    pub eip: u32,
    /// current CS
    pub cs: u32,
    /// current eflag
    pub eflags: Eflags,
    /// current esp
    pub esp: u32,
    /// current SS
    pub ss: u32,
}

use core::fmt::{self, Debug};

/// Debug boilerplate for CpuState:
/// SS & ESP may be out of kernel_stack bound, display it manually after ensure your from ring3
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Debug for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "eflags: {:X?} cs: 0x{:X?} eip: 0x{:X?} err_code: 0x{:X?} isr: {:?}
ds: 0x{:X?} es: 0x{:X?} fs: 0x{:X?} gs: 0x{:X?}
{:X?}",
               self.eflags, self.cs as u8, self.eip, self.err_code_reserved, self.cpu_isr_reserved,
               self.ds as u8, self.es as u8, self.fs as u8, self.gs as u8, self.registers)
    }
}

/// Declaration of shared Process trait. Kernel and User processes must implements these methods
pub trait Process {
    /// Return a new process
    unsafe fn new(
        origin: ProcessOrigin,
        arguments: Option<ProcessArguments>,
    ) -> SysResult<Box<Self>>;
    /// TSS initialisation method (necessary for ring3 switch)
    unsafe fn init_tss(&self);
    /// Start the process
    unsafe fn start(&self) -> !;
    /// Switch to the current process PD
    unsafe fn context_switch(&self);
}

/// This structure represents an entire process
pub struct UserProcess {
    /// kernel stack
    pub kernel_stack: Vec<u8>,
    /// Current process ESP on kernel stack
    pub kernel_esp: u32,
    /// Page directory of the process
    pub virtual_allocator: Arc<DeadMutex<AddressSpace>>,
    /// Symbol Table of a user process
    pub symbol_table: Option<Arc<SymbolTable>>,
}

/// This structure represents an entire kernel process
pub struct KernelProcess {
    /// kernel stack
    #[allow(dead_code)]
    kernel_stack: Vec<u8>,
    /// Current process ESP on kernel stack
    pub kernel_esp: u32,
}

/// Debug boilerplate for UserProcess
impl core::fmt::Debug for UserProcess {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:#X?} {:#X?} and a kernel_stack",
            unsafe { *(self.kernel_esp as *const CpuState) },
            self.virtual_allocator
        )
    }
}

/// Debug boilerplate for KernelProcess
impl core::fmt::Debug for KernelProcess {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:#X?} and a kernel_stack", unsafe {
            *(self.kernel_esp as *const CpuState)
        })
    }
}

/// This enum describe the origin of the process
#[allow(unused)]
pub enum ProcessOrigin<'a> {
    /// ELF file
    Elf(&'a [u8]),
    /// Just a dummy function
    Raw(*const u8, usize),
}

/// Main implementation of UserProcess
impl UserProcess {
    const RING0_STACK_SEGMENT: u16 = 0x18;

    const RING3_CODE_SEGMENT: u32 = 0x20;
    const RING3_DATA_SEGMENT: u32 = 0x28;
    const RING3_STACK_SEGMENT: u32 = 0x30;
    const RING3_DPL: u32 = 0b11;

    const RING3_RAW_PROCESS_MAX_SIZE: NbrPages = NbrPages::_64K;
    const RING3_PROCESS_STACK_SIZE: NbrPages = NbrPages::_64K;
    const RING3_PROCESS_KERNEL_STACK_SIZE: NbrPages = NbrPages::_128K;

    pub fn sys_clone(
        &self,
        kernel_esp: u32,
        child_stack: *const c_void,
        flags: CloneFlags,
    ) -> SysResult<Box<Self>> {
        // Create the child kernel stack
        let mut child_kernel_stack = try_vec![0; Self::RING3_PROCESS_KERNEL_STACK_SIZE.into()]?;
        assert!(
            child_kernel_stack.as_ptr() as usize
                & (Self::RING3_PROCESS_KERNEL_STACK_SIZE.to_bytes() - 1)
                == 0
        );
        child_kernel_stack
            .as_mut_slice()
            .copy_from_slice(self.kernel_stack.as_slice());

        // Set the kernel ESP of the child. Relative to kernel ESP of the father
        let child_kernel_esp =
            kernel_esp - self.kernel_stack.as_ptr() as u32 + child_kernel_stack.as_ptr() as u32;

        // Mark child syscall return as 0
        let child_cpu_state: *mut CpuState = child_kernel_esp as *mut CpuState;
        unsafe {
            (*child_cpu_state).registers.eax = 0;
            if !child_stack.is_null() {
                (*child_cpu_state).registers.esp = child_stack as u32;
                (*child_cpu_state).esp = child_stack as u32;
            }
        }

        Ok(Box::try_new(Self {
            kernel_stack: child_kernel_stack,
            kernel_esp: child_kernel_esp,
            virtual_allocator: if flags.contains(CloneFlags::VM) {
                self.virtual_allocator.clone()
            } else {
                // TODO: change that to Arc::try_new
                Arc::try_new(DeadMutex::new(self.virtual_allocator.lock().fork()?))?
            },
            symbol_table: self.symbol_table.as_ref().map(|elem| elem.clone()),
        })?)
    }
    pub fn get_virtual_allocator(&self) -> DeadMutexGuard<AddressSpace> {
        self.virtual_allocator.lock()
    }
}

/// Main implementation of KernalProcess
impl KernelProcess {
    const RING0_CODE_SEGMENT: u32 = 0x08;
    const RING0_DATA_SEGMENT: u32 = 0x10;
    const RING0_STACK_SEGMENT: u32 = 0x18;
    const RING0_DPL: u32 = 0b0;

    const KERNEL_RAW_PROCESS_MAX_SIZE: NbrPages = NbrPages::_64K;
    const KERNEL_PROCESS_STACK_SIZE: NbrPages = NbrPages::_64K;
}

/// Main implementation of process trait for UserProcess
impl Process for UserProcess {
    unsafe fn new(
        origin: ProcessOrigin,
        arguments: Option<ProcessArguments>,
    ) -> SysResult<Box<Self>> {
        // Store kernel CR3
        let old_cr3 = _read_cr3();
        // Create the process Page directory
        let mut virtual_allocator = AddressSpace::try_new()?;
        // Switch to this process Page Directory
        virtual_allocator.context_switch();

        let (eip, symbol_table) = match origin {
            ProcessOrigin::Elf(content) => {
                // Parse Elf and generate stuff
                let elf = load_elf(content);
                for h in &elf.program_header_table {
                    if h.segment_type == SegmentType::Load {
                        let segment = {
                            virtual_allocator.alloc_on(
                                h.vaddr as *mut u8,
                                h.memsz as usize,
                                AllocFlags::USER_MEMORY,
                            )?;
                            slice::from_raw_parts_mut(h.vaddr as usize as *mut u8, h.memsz as usize)
                        };
                        segment[0..h.filez as usize].copy_from_slice(
                            &content[h.offset as usize..h.offset as usize + h.filez as usize],
                        );
                        // With BSS (so a NOBITS section), the memsz value exceed the filesz. Setting next bytes as 0
                        segment[h.filez as usize..h.memsz as usize]
                            .as_mut_ptr()
                            .write_bytes(0, h.memsz as usize - h.filez as usize);
                        // Modify the rights on pages by following the ELF specific restrictions
                        virtual_allocator
                            .change_range_page_entry(
                                Page::containing(Virt(h.vaddr as usize)),
                                (h.memsz as usize).into(),
                                &mut |entry: &mut Entry| {
                                    *entry |= Entry::from(
                                        Into::<AllocFlags>::into(h.flags) | AllocFlags::USER_MEMORY,
                                    )
                                },
                            )
                            .expect("page must have been alloc by alloc on");
                    }
                }
                (
                    elf.header.entry_point as u32,
                    match SymbolTable::try_new(content).ok() {
                        Some(elem) => Some(Arc::try_new(elem)?),
                        None => None,
                    },
                )
            }
            ProcessOrigin::Raw(code, code_len) => {
                // Allocate one page for code segment of the Dummy process
                let base_addr = virtual_allocator
                    .alloc(Self::RING3_RAW_PROCESS_MAX_SIZE, AllocFlags::USER_MEMORY)?;
                // Copy the code segment
                base_addr.copy_from(code, code_len);
                (base_addr as u32, None)
            }
        };

        // Allocate the kernel stack of the process
        let kernel_stack = try_vec![0; Self::RING3_PROCESS_KERNEL_STACK_SIZE.into()]?;
        assert!(
            kernel_stack.as_ptr() as usize & (Self::RING3_PROCESS_KERNEL_STACK_SIZE.to_bytes() - 1)
                == 0
        );

        // Mark the first entry of the kernel stack as read-only, its make an Triple fault when happened
        virtual_allocator.change_flags_page_entry(
            Virt(kernel_stack.as_ptr() as usize).into(),
            AllocFlags::READ_ONLY | AllocFlags::KERNEL_MEMORY,
        );

        // Generate the start kernel ESP of the new process
        let kernel_esp = kernel_stack.as_ptr().add(
            Into::<usize>::into(Self::RING3_PROCESS_KERNEL_STACK_SIZE)
                - core::mem::size_of::<CpuState>(),
        ) as u32;

        // Allocate one page for stack segment of the process
        let stack_addr =
            virtual_allocator.alloc(Self::RING3_PROCESS_STACK_SIZE, AllocFlags::USER_MEMORY)?;

        // Mark the first entry of the user stack as read-only, this prevent user stack overflow
        virtual_allocator.change_flags_page_entry(
            Virt(stack_addr as usize).into(),
            AllocFlags::READ_ONLY | AllocFlags::USER_MEMORY,
        );

        // stack go downwards set esp to the end of the allocation
        let mut esp = stack_addr.add(Self::RING3_PROCESS_STACK_SIZE.into()) as u32;

        let (mut eax, mut ebx, mut ecx) = (0, 0, 0);

        // Assign arguments of the main function
        if let Some(arguments) = arguments {
            let align = 4;

            // Set the argc argument: EAX
            eax = arguments.argv.len() as u32;

            // Set the argv argument: EBX
            esp -= arguments.argv.get_serialized_len(align).expect("WTF") as u32;
            ebx = arguments
                .argv
                .serialize(align, esp as *mut c_char)
                .expect("WTF") as u32;

            // set the envp argument: ECX
            esp -= arguments.envp.get_serialized_len(align).expect("WTF") as u32;
            ecx = arguments
                .envp
                .serialize(align, esp as *mut c_char)
                .expect("WTF") as u32;
        }

        // Create the process identity
        let cpu_state: CpuState = CpuState {
            stack_reserved: 0,
            fx_region: Default::default(),
            fx_padding: 0,
            registers: BaseRegisters {
                esp,
                eax,
                ebx,
                ecx,
                ..Default::default()
            }, // Be carefull, never trust ESP
            ds: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
            es: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
            fs: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
            gs: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
            cpu_isr_reserved: 0,
            err_code_reserved: 0,
            eip,
            cs: Self::RING3_CODE_SEGMENT + Self::RING3_DPL,
            eflags: Eflags::get_eflags().set_interrupt_flag(true), // TODO: Change that get_eflags is for sure an error
            esp,
            ss: Self::RING3_STACK_SEGMENT + Self::RING3_DPL,
        };

        // Fill the kernel stack of the new process with start cpu states.
        *(kernel_esp as *mut CpuState) = cpu_state;

        // Re-enable kernel virtual space memory
        _enable_paging(old_cr3);

        Ok(Box::try_new(UserProcess {
            kernel_stack,
            kernel_esp,
            virtual_allocator: Arc::try_new(DeadMutex::new(virtual_allocator))?,
            symbol_table,
        })?)
    }

    unsafe fn init_tss(&self) {
        TSS.lock().init(
            self.kernel_stack
                .as_ptr()
                .add(Self::RING3_PROCESS_KERNEL_STACK_SIZE.into()) as u32,
            Self::RING0_STACK_SEGMENT,
        );
    }

    unsafe fn context_switch(&self) {
        // Switch to the new process PD
        self.virtual_allocator.lock().context_switch();
        // Re-init the TSS block for the new process
        self.init_tss();
    }

    unsafe fn start(&self) -> ! {
        self.context_switch();

        // Launch the ring3 process on its own kernel stack
        _start_process(self.kernel_esp)
    }
}

// Note: It is very really tricky to exit from a kernel process about possibles memory leaks or code corruption
// The using of Syscalls from a kernel process can lead to a lot of undefined behavior. please avoid it
// Maybe we need to check CS of the caller of the TSS segment in the syscall handler to unallow use of them.
/// Main implementation of a KernelProcess
impl Process for KernelProcess {
    unsafe fn new(
        origin: ProcessOrigin,
        arguments: Option<ProcessArguments>,
    ) -> SysResult<Box<Self>> {
        if let Some(_arguments) = arguments {
            panic!("Giving process arguments to a kernel process is a non-implemented feature !");
        }
        let eip = match origin {
            ProcessOrigin::Elf(_content) => {
                unimplemented!();
            }
            ProcessOrigin::Raw(code, code_len) => {
                // Allocate a chunk of memory for the process code
                let base_addr = KERNEL_VIRTUAL_PAGE_ALLOCATOR
                    .as_mut()
                    .unwrap()
                    .alloc(Self::KERNEL_RAW_PROCESS_MAX_SIZE, AllocFlags::KERNEL_MEMORY)?
                    .to_addr()
                    .0 as *mut u8;
                // Copy the code segment
                base_addr.copy_from(code, code_len);
                base_addr as u32
            }
        };

        // Allocate the kernel stack of the process
        let kernel_stack = try_vec![0; Self::KERNEL_PROCESS_STACK_SIZE.into()]?;
        assert!(
            kernel_stack.as_ptr() as usize & (Self::KERNEL_PROCESS_STACK_SIZE.to_bytes() - 1) == 0
        );

        // Mark the first entry of the kernel stack as read-only, its make an Triple fault when happened
        KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .change_flags_page_entry(
                Virt(kernel_stack.as_ptr() as usize).into(),
                AllocFlags::READ_ONLY | AllocFlags::KERNEL_MEMORY,
            );

        // Generate the start kernel ESP of the new process
        let kernel_esp = kernel_stack.as_ptr().add(
            Into::<usize>::into(Self::KERNEL_PROCESS_STACK_SIZE) - core::mem::size_of::<CpuState>(),
        ) as u32;

        // Create the process identity
        let cpu_state: CpuState = CpuState {
            stack_reserved: 0,
            fx_region: Default::default(),
            fx_padding: 0,
            registers: BaseRegisters {
                esp: kernel_esp,
                ..Default::default()
            }, // Be carefull, never trust ESP
            ds: Self::RING0_DATA_SEGMENT + Self::RING0_DPL,
            es: Self::RING0_DATA_SEGMENT + Self::RING0_DPL,
            fs: Self::RING0_DATA_SEGMENT + Self::RING0_DPL,
            gs: Self::RING0_DATA_SEGMENT + Self::RING0_DPL,
            cpu_isr_reserved: 0,
            err_code_reserved: 0,
            eip,
            cs: Self::RING0_CODE_SEGMENT + Self::RING0_DPL,
            eflags: Eflags::get_eflags().set_interrupt_flag(true),
            esp: kernel_esp,
            ss: Self::RING0_STACK_SEGMENT + Self::RING0_DPL,
        };
        // Fill the kernel stack of the new process with start cpu states.
        (kernel_esp as *mut u8).copy_from(
            &cpu_state as *const _ as *const u8,
            core::mem::size_of::<CpuState>(),
        );

        Ok(Box::try_new(KernelProcess {
            kernel_stack,
            kernel_esp,
        })?)
    }

    unsafe fn init_tss(&self) {
        // Initialize the TSS segment is not necessary for a kernel process
    }

    unsafe fn start(&self) -> ! {
        // Launch the kerne; process on its own kernel stack
        _start_process(self.kernel_esp)
    }

    unsafe fn context_switch(&self) {
        // Context_switch is not necessary for a kernel process
    }
}

/// Return the current ring relative to cpu_state: context_ptr must be right
pub unsafe fn get_ring(context_ptr: u32) -> PrivilegeLevel {
    let cpu_state = context_ptr as *const CpuState;
    (((*cpu_state).cs & 0b11) as u8).into()
}

use super::IpcResult;
use super::{thread_group::Credentials, vfs::Path};
use core::convert::TryFrom;

/// Return a file content using raw ext2 methods
pub fn get_file_content(cwd: &Path, creds: &Credentials, pathname: &str) -> SysResult<Vec<u8>> {
    // TODO: REMOVE THIS SHIT
    let path = super::vfs::Path::try_from(pathname)?;
    let mode =
        super::vfs::FilePermissions::from_bits(0o777).expect("file permission creation failed");
    let flags = libc_binding::OpenFlags::O_RDONLY;
    let file_operator = match super::vfs::VFS.lock().open(cwd, creds, path, flags, mode)? {
        IpcResult::Done(file_operator) => file_operator,
        IpcResult::Wait(file_operator, _) => file_operator,
    };
    let mut file = file_operator.lock();

    let mut stat = unsafe { core::mem::uninitialized() };
    file.fstat(&mut stat)?;

    let file_size = stat.st_size;

    let mut v: Vec<u8> = try_vec![0; stat.st_size as usize]?;

    let len = file.read(&mut v)?.expect("execve: file is bullshit");

    if len != file_size as u32 {
        Err(Errno::EIO)
    } else {
        Ok(v)
    }
}
