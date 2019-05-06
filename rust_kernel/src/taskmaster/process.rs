//! This file contains the process description

pub mod tss;

// use crate::elf_loader::load_elf;
use crate::memory::allocator::VirtualPageAllocator;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages};
use crate::registers::Eflags;
use crate::system::BaseRegisters;

extern "C" {
    fn _launch_process(cpu_state: *const CpuState) -> !;
}

/// Represent all cpu state needed to continue the execution of a process
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CpuState {
    /// current registers
    registers: BaseRegisters,
    /// current data DS
    ds: u32,
    /// current data ES
    es: u32,
    /// current data FS
    fs: u32,
    /// current data GS
    gs: u32,
    /// current eip
    eip: u32,
    /// current CS
    cs: u32,
    /// current eflag
    eflags: Eflags,
    /// current esp
    esp: u32,
    /// current SS
    ss: u32,
}

/// This structure represents an entire process
#[derive(Debug)]
pub struct Process {
    /// represent the state of the processor when the process was last stopped
    cpu_state: CpuState,
    /// Page directory of the process
    pub virtual_allocator: VirtualPageAllocator,
}

/// Main implementation of Process
impl Process {
    const RING3_CODE_SEGMENT: u32 = 0x20;
    const RING3_DATA_SEGMENT: u32 = 0x28;
    const RING3_STACK_SEGMENT: u32 = 0x30;
    const RING3_DPL: u32 = 0b11;
    const RING3_PROCESS_MAX_SIZE: NbrPages = NbrPages::_1MB;

    /// Create a new process
    pub unsafe fn new(code: *const u8, code_len: usize) -> Self {
        // Store kernel CR3
        let old_cr3 = _read_cr3();
        // Create a Dummy process Page directory
        let mut v = VirtualPageAllocator::new_for_process();
        // Switch to this process Page Directory
        v.context_switch();

        // Allocate one page for code segment of the Dummy process
        let base_addr = v.alloc(Self::RING3_PROCESS_MAX_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
        // stack go downwards set esp to the end of the allocation
        let esp = base_addr.add(Self::RING3_PROCESS_MAX_SIZE.into()) as u32;
        let res = Self {
            cpu_state: CpuState {
                registers: BaseRegisters { esp, ..Default::default() }, // Be carefull, never trust ESP
                ds: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
                es: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
                fs: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
                gs: Self::RING3_DATA_SEGMENT + Self::RING3_DPL,
                eip: base_addr as u32,
                cs: Self::RING3_CODE_SEGMENT + Self::RING3_DPL,
                eflags: Eflags::get_eflags().set_interrupt_flag(true),
                esp,
                ss: Self::RING3_STACK_SEGMENT + Self::RING3_DPL,
            },
            virtual_allocator: v,
        };
        // Copy the code segment
        base_addr.copy_from(code, code_len);
        _enable_paging(old_cr3);
        // When non-fork, return to Kernel PD, when forking, return to father PD
        res
    }
    /*
    /// Load a real process
    pub unsafe fn load(eflags: Eflags) -> crate::memory::tools::Result<Self> {
        let old_cr3 = _read_cr3();
        let mut v = VirtualPageAllocator::new_for_process();
        v.context_switch();

        let content: &[u8] = &include_bytes!("./Charles")[..];
        let elf = load_elf();
        for h in &elf.program_header_table {
            use core::slice;
            use elf_loader::SegmentType;
            if h.segment_type == SegmentType::Load {
                println!("{:X?}", h);
                let segment = {
                    let _segment_addr = v
                        .alloc_on(Page::containing(Virt(h.vaddr as usize)), (h.memsz as usize).into(), h.flags.into())?
                        .to_addr()
                        .0 as *mut u8;
                    slice::from_raw_parts_mut(h.vaddr as usize as *mut u8, h.memsz as usize)
                };

                println!("segment: {:X?}", segment.as_ptr());
                for (dest, src) in
                    segment.iter_mut().zip(content[h.offset as usize..h.offset as usize + h.filez as usize].iter())
                {
                    *dest = *src;
                }
            }
        }

        let base_stack = v.alloc(STACK_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
        let res = Self {
            cpu_state: CpuState {
                eip: elf.header.entry_point,
                // stack go downwards set esp to the end of the allocation
                esp: dbg_hex!(base_stack.add(STACK_SIZE.into()) as u32 - 16),
                registers: Default::default(),
                segment: 0x8,
                eflags,
            },
            base_stack: base_stack as u32 - 16,
            virtual_allocator: v,
            pid: scheduler::get_available_pid(),
            state: State::Running,
        };

        _enable_paging(old_cr3);

        Ok(res)
    }
    */
    /// Launch a process
    pub unsafe fn launch(&self) -> ! {
        // Switch to process Page Directory
        self.virtual_allocator.context_switch();

        // Launch the ring3 process
        _launch_process(&self.cpu_state)
    }

    /// Fork a process
    #[allow(dead_code)]
    pub fn fork(&self) -> crate::memory::tools::Result<Self> {
        let mut child = Self { cpu_state: self.cpu_state, virtual_allocator: self.virtual_allocator.fork()? };
        child.cpu_state.registers.eax = 0;
        Ok(child)
    }

    /// Destroy a process
    #[allow(dead_code)]
    pub fn exit(&mut self) {
        // TODO: free all memory allocations by following the virtual_allocator keys 4mb-3g area
        unimplemented!();
        // drop(*self);
    }

    /// Save all the cpu_state of a process
    pub fn set_process_state(&mut self, cpu_state: CpuState) {
        self.cpu_state = cpu_state;
    }

    /// Get all the cpu_state of a process
    pub fn get_process_state(&self) -> CpuState {
        self.cpu_state
    }
}
