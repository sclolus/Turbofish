use crate::memory::allocator::VirtualPageAllocator;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages};
use crate::registers::Eflags;
use crate::system::BaseRegisters;

pub mod scheduler;
pub mod tests;
pub mod tss;

/// state of a process
#[derive(Debug, Clone)]
pub enum State {
    New,
    Terminated { status: i32 },
    Running,
    Waiting,
}

/// Represent all cpu state needed to continue the execution of a process
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CpuState {
    /// current registers
    pub registers: BaseRegisters,
    /// current data DS
    pub ds: u32,
    /// current data ES
    pub es: u32,
    /// current data FS
    pub fs: u32,
    /// current data GS
    pub gs: u32,
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

/// This structure represent an entire process
#[derive(Debug)]
pub struct Process {
    /// represent the state of the processor when the process was last stoped
    pub cpu_state: CpuState,
    /// allocator for the process
    pub virtual_allocator: VirtualPageAllocator,
    /// The pid
    pub pid: u32,
    /// The state of the process
    pub state: State,
    /// Type of the process, ring3 or kernel
    pub process_type: ProcessType,
}

/// Ring3 basic or a kernel process
#[derive(Debug, Copy, Clone)]
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
    pub unsafe fn new(code: *const u8, code_len: Option<usize>, process_type: ProcessType) -> Self {
        match process_type {
            ProcessType::Kernel => {
                let old_cr3 = _read_cr3();
                // Ceate a Dummy process Page directory
                let mut v = VirtualPageAllocator::new_for_process();
                // Switch to this process Page Directory
                v.context_switch();
                // Allocate a chunk for process stack (Ring0 process dont use TSS segment so it share stack when IRQ pop)
                let stack_addr =
                    v.alloc(Self::KERNEL_PROCESS_STACK_SIZE, AllocFlags::KERNEL_MEMORY).unwrap().to_addr().0 as *mut u8;
                // stack go downwards set esp to the end of the allocation
                let esp = stack_addr.add(Self::KERNEL_PROCESS_STACK_SIZE.into()) as u32;
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
                    virtual_allocator: v,
                    pid: scheduler::get_available_pid(), // TODO: Is it a correct design that scheduler provide PID ?
                    state: State::New,
                    process_type,
                };
                // When non-fork, return to Kernel PD, when forking, return to father PD
                _enable_paging(old_cr3);
                res
            }
            ProcessType::Ring3 => {
                let old_cr3 = _read_cr3();
                // Ceate a Dummy process Page directory
                let mut v = VirtualPageAllocator::new_for_process();
                // Switch to this process Page Directory
                v.context_switch();
                // Allocate one page for code segment of the Dummy process
                let base_addr =
                    v.alloc(Self::RING3_PROCESS_MAX_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
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
                    pid: scheduler::get_available_pid(), // TODO: Is it a correct design that scheduler provide PID ?
                    state: State::New,
                    process_type,
                };
                // Copy the code segment
                ft_memcpy(base_addr, code, code_len.unwrap());
                // When non-fork, return to Kernel PD, when forking, return to father PD
                _enable_paging(old_cr3);
                res
            }
        }
    }

    /// Launch a process
    pub unsafe fn launch(&self) {
        // Switch to process Page Directory
        self.virtual_allocator.context_switch();
        match self.process_type {
            ProcessType::Kernel => {
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
                let len = core::mem::size_of::<CpuState>();
                let esp_start = self.cpu_state.esp - len as u32;
                ft_memcpy(esp_start as *mut u8, &self.cpu_state as *const _ as *const u8, len);

                // Launch the process
                _launch_process(esp_start as *const CpuState);
            }
            ProcessType::Ring3 => {
                _launch_process(&self.cpu_state);
            }
        }
    }

    // pub fn fork(&self) -> crate::memory::tools::Result<Self> {
    //     let mut child = Self {
    //         pid: scheduler::get_available_pid(),
    //         cpu_state: self.cpu_state,
    //         base_stack: self.base_stack,
    //         state: State::Running,
    //         virtual_allocator: self.virtual_allocator.fork()?,
    //     };
    //     child.cpu_state.registers.eax = 0;
    //
    //     Ok(child)
    // }

    // pub fn exit(&mut self, status: i32) {
    //     self.state = State::Terminated { status };
    // TODO: free resource
    // }
}

extern "C" {
    fn _launch_process(cpu_state: *const CpuState);
    fn ft_memcpy(dst: *mut u8, src: *const u8, len: usize);
}
