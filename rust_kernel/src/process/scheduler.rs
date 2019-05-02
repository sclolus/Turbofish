use crate::process::Process;
use crate::registers::Eflags;
use crate::spinlock::Spinlock;
use crate::system::BaseRegisters;
use alloc::vec;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
// use hashmap_core::map::HashMap;
use lazy_static::lazy_static;

extern "C" {
    fn _process_a();
    fn _process_b();
    /// set all processor state to its arguments and iret to eip
    fn _switch_process(eflags: u32, segment: u32, eip: u32, esp: u32, registers: BaseRegisters) -> !;
    static mut SCHEDULER_ACTIVE: bool;
}

type Pid = u32;

/// the pit handler
#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(
    old_eip: u32,
    old_segment: u32,
    old_eflags: u32,
    old_esp: u32,
    registers: BaseRegisters,
) -> ! {
    let mut scheduler = SCHEDULER.lock();
    scheduler.save_process_state(old_eip, old_segment, old_eflags, old_esp, registers);
    scheduler.schedule()
}

pub struct Scheduler {
    /// contains pids of all runing process
    running_process: Vec<Pid>,
    /// contains a hashmap of pid, process
    all_process: HashMap<Pid, Process>,
    /// index in the vector of the current running process
    //curr_process_pid: Pid,
    curr_process_index: usize,
    // TODO: remove that, it is just use for starting the first process
    no_process: bool,
}

impl Scheduler {
    unsafe fn new() -> Self {
        let all_process = {
            let mut a = HashMap::new();
            let p = Process::new(_process_a, Eflags::get_eflags().set_interrupt_flag(true));
            dbg!(&p);
            a.insert(p.pid, p);
            let p = Process::new(_process_b, Eflags::get_eflags().set_interrupt_flag(true));
            dbg!(&p);
            a.insert(p.pid, p);
            a
        };
        Self {
            running_process: all_process.keys().map(|x| *x).collect(),
            all_process,
            curr_process_index: 0,
            no_process: true,
        }
    }

    fn save_process_state(
        &mut self,
        old_eip: u32,
        old_segment: u32,
        old_eflags: u32,
        old_esp: u32,
        registers: BaseRegisters,
    ) {
        let eflags = crate::registers::Eflags::new(old_eflags);
        // eprintln!(
        //     "saving process with: eip:{:X?} esp:{:X?} reg:{:X?}\n eflags: {}",
        //     old_eip, old_esp, registers, eflags
        // );
        if self.no_process {
            self.no_process = false;
            return;
        }
        let curr_process: &mut Process =
            self.all_process.get_mut(&self.running_process[self.curr_process_index]).unwrap();
        curr_process.eip = old_eip;
        curr_process.esp = old_esp;
        curr_process.registers = registers;
        curr_process.eflags = eflags;
        curr_process.segment = old_segment;
    }

    fn schedule(&mut self) -> ! {
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
        let next_process: &Process = self.all_process.get(&self.running_process[self.curr_process_index]).unwrap();
        // eprintln!(
        //     "switch to eip:{:X?} esp:{:X?} reg:{:X?}\n eflags: {}",
        //     next_process.eip, next_process.esp, next_process.registers, next_process.eflags
        // );
        unsafe {
            next_process.virtual_allocator.context_switch();
            SCHEDULER.force_unlock();
            _switch_process(
                next_process.eflags.inner(),
                next_process.segment,
                next_process.eip,
                next_process.esp,
                next_process.registers,
            );
        }
    }

    pub fn fork(&mut self) {
        unimplemented!();
    }

    pub fn exit(&mut self, status: i32) {
        unimplemented!();
        // self.process[self.curr_process_index].exit(status);
        // self.curr_process_index = (self.curr_process_index + 1) % self.process.len();
    }
}

lazy_static! {
    pub static ref SCHEDULER: Spinlock<Scheduler> = Spinlock::new(unsafe { Scheduler::new() });
}

use core::sync::atomic::{AtomicU32, Ordering};

/// represent the greatest available pid
static MAX_PID: AtomicU32 = AtomicU32::new(0);

/// get the next available pid for a new process
pub fn get_available_pid() -> u32 {
    //TODO: handle when overflow to 0
    MAX_PID.fetch_add(1, Ordering::Relaxed)
}

pub fn init() {
    unsafe {
        SCHEDULER_ACTIVE = true;
    }
}

/// stupid kernel space process a
#[no_mangle]
extern "C" fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

/// stupid kernel space process b
#[no_mangle]
extern "C" fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}
