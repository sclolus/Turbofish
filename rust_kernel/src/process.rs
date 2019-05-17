use crate::memory::allocator::VirtualPageAllocator;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages};
use crate::registers::Eflags;
use crate::system::BaseRegisters;

pub mod scheduler;
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
}

extern "C" {
    fn ft_memcpy(dst: *mut u8, src: *const u8, len: usize);
}

impl Process {
    const PROCESS_MAX_SIZE: NbrPages = NbrPages::_1MB;

    /// create a new process
    pub unsafe fn new(code: *const u8, code_len: usize) -> Self {
        let old_cr3 = _read_cr3();
        // Ceate a Dummy process Page directory
        let mut v = VirtualPageAllocator::new_for_process();
        // Switch to this process Page Directory
        v.context_switch();
        // Allocate one page for code segment of the Dummy process
        let base_addr = v.alloc(Self::PROCESS_MAX_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
        // stack go downwards set esp to the end of the allocation
        let esp = base_addr.add(Self::PROCESS_MAX_SIZE.into()) as u32;
        let res = Self {
            cpu_state: CpuState {
                registers: BaseRegisters { esp, ..Default::default() }, // Be carefull, never trust ESP
                ds: 0x28 + 3,
                es: 0x28 + 3,
                fs: 0x28 + 3,
                gs: 0x28 + 3,
                eip: base_addr as u32,
                cs: 0x20 + 3,
                eflags: Eflags::get_eflags().set_interrupt_flag(true),
                esp,
                ss: 0x30 + 3,
            },
            virtual_allocator: v,
            pid: scheduler::get_available_pid(), // TODO: Is it a correct design that scheduler provide PID ?
            state: State::Running,
        };
        // Copy the code segment
        ft_memcpy(base_addr, code, code_len);
        // When non-fork, return to Kernel PD, when forking, return to father PD
        _enable_paging(old_cr3);
        res
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
