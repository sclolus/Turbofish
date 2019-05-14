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
    Terminated { status: i32 },
    Running,
    Waiting,
}

/// represent all cpu state needed to continue the execution of a process
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CpuState {
    /// current eip
    pub eip: u32,
    /// current segment of code
    pub segment: u32,
    /// current eflags
    pub eflags: Eflags,
    /// current esp
    pub esp: u32,
    /// current registers
    pub registers: BaseRegisters,
}

#[derive(Debug)]
pub struct Process {
    /// pointer to the base of the stack
    pub base_stack: u32,
    /// represent the state of the processor when the process was last stoped
    pub cpu_state: CpuState,
    /// allocator for the process
    pub virtual_allocator: VirtualPageAllocator,
    /// The pid
    pub pid: u32,
    pub state: State,
}

const STACK_SIZE: NbrPages = NbrPages::_1MB;

impl Process {
    /// create a new process wich will execute f, and start with eflags
    pub unsafe fn new(f: fn(), eflags: Eflags) -> Self {
        let old_cr3 = _read_cr3();
        let mut v = VirtualPageAllocator::new_for_process();
        v.context_switch();
        let base_stack = v.alloc(STACK_SIZE, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
        let res = Self {
            cpu_state: CpuState {
                eip: f as u32,
                // stack go downwards set esp to the end of the allocation
                esp: base_stack.add(STACK_SIZE.into()) as u32,
                registers: Default::default(),
                segment: 0x8,
                eflags,
            },
            base_stack: base_stack as u32,
            virtual_allocator: v,
            pid: scheduler::get_available_pid(),
            state: State::Running,
        };
        _enable_paging(old_cr3);
        res
    }

    pub fn fork(&self) -> crate::memory::tools::Result<Self> {
        let mut child = Self {
            pid: scheduler::get_available_pid(),
            cpu_state: self.cpu_state,
            base_stack: self.base_stack,
            state: State::Running,
            virtual_allocator: self.virtual_allocator.fork()?,
        };
        child.cpu_state.registers.eax = 0;

        Ok(child)
    }
    pub fn exit(&mut self, status: i32) {
        self.state = State::Terminated { status };
        // TODO: free resource
    }
}
