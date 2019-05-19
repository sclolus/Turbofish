//! This file contains the process description

pub mod tss;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::memory::allocator::VirtualPageAllocator;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages};
use crate::registers::Eflags;
use crate::system::BaseRegisters;

extern "C" {
    fn _launch_process(cpu_state: *const CpuState);
    fn ft_memcpy(dst: *mut u8, src: *const u8, len: usize);
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
    pub esp: u32,
    /// current SS
    ss: u32,
}

/// Just a boilerplate for fun
type KernelStack = Vec<u8>;

/// This structure represents an entire process
pub struct Process {
    /// represent the state of the processor when the process was last stopped
    cpu_state: CpuState,
    /// Page directory of the process (only ring 3 process implements that)
    pub virtual_allocator: Option<VirtualPageAllocator>,
    /// Kernel Stack of each processes (TODO: Implenment it in ring3 for premptif scheduler)
    pub kernel_stack: KernelStack,
    /// Type of the process, ring3 or kernel
    pub process_type: ProcessType,
}

/// Custom debug definition for Process
impl core::fmt::Debug for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "cpu_state: {:#X?}\nproces_type: {:?}", self.cpu_state, self.process_type)
    }
}

/// Ring3 basic or a kernel process
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProcessType {
    Kernel,
    Ring3,
}

/// Main implementation of Process
impl Process {
    const KERNEL_CODE_SEGMENT: u32 = 0x8;
    const KERNEL_DATA_SEGMENT: u32 = 0x10;
    const KERNEL_STACK_SEGMENT: u32 = 0x18;
    const KERNEL_DPL: u32 = 0b00;
    const KERNEL_PROCESS_STACK_SIZE: NbrPages = NbrPages::_1MB;
    const RING3_CODE_SEGMENT: u32 = 0x20;
    const RING3_DATA_SEGMENT: u32 = 0x28;
    const RING3_STACK_SEGMENT: u32 = 0x30;
    const RING3_DPL: u32 = 0b11;
    const RING3_PROCESS_MAX_SIZE: NbrPages = NbrPages::_1MB;

    /// Create a new process
    pub unsafe fn new(code: *const u8, code_len: Option<usize>, process_type: ProcessType) -> Box<Self> {
        eprintln!("begin");
        if process_type == ProcessType::Kernel {
            let kernel_stack: KernelStack = vec![0; 1 << 20];

            // stack go downwards set esp to the end of the allocation
            let esp = (&kernel_stack[0] as *const u8).add(Self::KERNEL_PROCESS_STACK_SIZE.into()) as u32
                - core::mem::size_of::<CpuState>() as u32;
            let res = Self {
                cpu_state: CpuState {
                    registers: BaseRegisters { esp, ..Default::default() }, // Be carefull, never trust ESP
                    ds: Self::KERNEL_DATA_SEGMENT + Self::KERNEL_DPL,
                    es: Self::KERNEL_DATA_SEGMENT + Self::KERNEL_DPL,
                    fs: Self::KERNEL_DATA_SEGMENT + Self::KERNEL_DPL,
                    gs: Self::KERNEL_DATA_SEGMENT + Self::KERNEL_DPL,
                    eip: code as u32,
                    cs: Self::KERNEL_CODE_SEGMENT + Self::KERNEL_DPL,
                    eflags: Eflags::get_eflags().set_interrupt_flag(true),
                    esp,
                    ss: Self::KERNEL_STACK_SEGMENT + Self::KERNEL_DPL,
                },
                virtual_allocator: None,
                kernel_stack,
                process_type,
            };

            /*
             * Kernel process dont use TSS segment, so there are many implications
             * - When a IRQ pop (syscall or schedule etc...), the stack pointer is not changed
             * - So each kernel process must have their own kernel stacks (like as preemptif scheme)
             *
             *                        Overview of Kernel Process Stack
             *   (<- to low addr)                                          esp_start        esp max
             *                                                                 |               |
             * <---------------------------------------------------------------v---------------|
             *                                                                 |     struct    |
             *                           <---- stack size ---->                |      cpu      |
             *                                                                 |     state     |
             * <---------------------------------------------------------------+---------------+
             *                                              inc esp while call +-------------> IRET
             * After IRET instruction, the process use his own stack according to his current ESP value
             * The struct cpu_state must be copied into the high part of the process stack to do the launch
             */

            // Initialise process by copying cpu_states into his own stack (prepare the first launch)
            ft_memcpy(esp as *mut u8, &res.cpu_state as *const _ as *const u8, core::mem::size_of::<CpuState>());
            // When non-fork, return to Kernel PD, when forking, return to father PD
            Box::new(res)
        } else {
            // Store kernel CR3
            let old_cr3 = _read_cr3();
            // Create a Dummy process Page directory
            let mut v = VirtualPageAllocator::new_for_process();
            // Switch to this process Page Directory
            v.context_switch();

            // Allocate one page for code segment of the Dummy process
            let base_addr =
                v.alloc(Self::RING3_PROCESS_MAX_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
            // stack go downwards set esp to the end of the allocation
            let kernel_stack: KernelStack = vec![0; 1 << 20];
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
                virtual_allocator: Some(v),
                kernel_stack,
                process_type,
            };
            // Copy the code segment
            ft_memcpy(base_addr, code, code_len.unwrap());
            _enable_paging(old_cr3);
            // When non-fork, return to Kernel PD, when forking, return to father PD
            Box::new(res)
        }
    }

    /// Launch a process
    pub unsafe fn launch(&self) {
        match self.process_type {
            ProcessType::Kernel => {
                // Launch the kernel process
                _launch_process(self.cpu_state.esp as *const CpuState);
            }
            ProcessType::Ring3 => {
                // Switch to process Page Directory
                self.virtual_allocator.as_ref().unwrap().context_switch();

                // Launch the ring3 process
                _launch_process(&self.cpu_state);
            }
        }
    }

    /// Fork a process
    #[allow(dead_code)]
    pub fn fork(&self) -> crate::memory::tools::Result<Box<Self>> {
        let mut child = Self {
            cpu_state: self.cpu_state,
            virtual_allocator: match self.virtual_allocator.as_ref() {
                Some(v) => Some(v.fork()?),
                None => None,
            },
            kernel_stack: vec![0; 1 << 20], // TODO, copy kernel stack content
            process_type: self.process_type,
        };
        child.cpu_state.registers.eax = 0;
        Ok(Box::new(child))
    }

    /// Destroy a process
    #[allow(dead_code)]
    pub fn exit(&mut self) {
        // TODO: free all memory allocations by following the virtual_allocator keys 4mb-3g area
        drop(self);
    }

    /// Save all the cpu_state of a process
    pub fn set_process_state(&mut self, cpu_state: *const CpuState) {
        self.cpu_state = unsafe { *cpu_state };
    }

    /// Get all the cpu_state of a process
    pub fn get_process_state(&self) -> *const CpuState {
        &self.cpu_state
    }
}
