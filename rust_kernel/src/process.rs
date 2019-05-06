use crate::memory::allocator::VirtualPageAllocator;
use crate::memory::mmu::{_enable_paging, _read_cr3};
use crate::memory::tools::{AllocFlags, NbrPages};
use crate::registers::Eflags;
use crate::system::BaseRegisters;
pub mod scheduler;

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

// tss unused for the moment
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
struct Tss {
    /*0x00*/ _reserved1: u16, link: u16,
    /*0x04*/ esp0: u32,
    /*0x08*/ _reserved2: u16, ss0: u16,
    /*0x0C*/ esp1: u32,
    /*0x10*/ _reserved3: u16, ss1: u16,
    /*0x14*/ esp2: u32,
    /*0x18*/ _reserved4: u16, ss2: u16,
    /*0x1C*/ cr3: u32,
    /*0x20*/ eip: u32,
    /*0x24*/ eflags: u32,
    /*0x28*/ eax: u32,
    /*0x2C*/ ecx: u32,
    /*0x30*/ edx: u32,
    /*0x34*/ ebx: u32,
    /*0x38*/ esp: u32,
    /*0x3C*/ ebp: u32,
    /*0x40*/ esi: u32,
    /*0x44*/ edi: u32,
    /*0x48*/ _reserved5: u16, es: u16,
    /*0x4C*/ _reserved6: u16, cs: u16,
    /*0x50*/ _reserved7: u16, ss: u16,
    /*0x54*/ _reserved8: u16, ds: u16,
    /*0x58*/ _reserved9: u16, fs: u16,
    /*0x5C*/ _reserved10: u16, gs: u16,
    /*0x60*/ _reserved11: u16, ldtr: u16,
}

extern "C" {
    pub fn load_tss(tss_gdt_descriptor: u8);
}
