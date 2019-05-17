use super::{CpuState, Process, ProcessType};
//use crate::registers::Eflags;
use crate::spinlock::Spinlock;
use alloc::vec;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
// use hashmap_core::map::HashMap;
use lazy_static::lazy_static;
// use crate::system::BaseRegisters;

extern "C" {
    static mut SCHEDULER_ACTIVE: bool;
}

type Pid = u32;

/// the pit handler
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(cpu_state: *mut CpuState) {
    // TODO: Put real content of the next process here (instead of simple copy)
    let c: CpuState = CpuState {
        registers: (*cpu_state).registers,
        ds: (*cpu_state).ds,
        es: (*cpu_state).es,
        fs: (*cpu_state).fs,
        gs: (*cpu_state).gs,
        eip: (*cpu_state).eip,
        cs: (*cpu_state).cs,
        eflags: (*cpu_state).eflags,
        esp: (*cpu_state).esp,
        ss: (*cpu_state).ss,
    };
    //eprintln!("{:#X?}", *cpu_state);
    // Associate next process content
    *cpu_state = c;

    /*
    let mut scheduler = SCHEDULER.lock();
    scheduler.save_process_state(cpu_state);
    scheduler.switch_next_process();
    scheduler.return_to_process()
    */
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
    // Start to schedule
    pub unsafe fn start() {
        SCHEDULER_ACTIVE = true;
    }

    // create a new scheduler for tests
    unsafe fn new() -> Self {
        let test_process = vec![
            Process::new(0 as *mut u8, Some(0), ProcessType::Ring3),
            // Process::new(process_a, Eflags::get_eflags().set_interrupt_flag(true)),
            // Process::new(process_b, Eflags::get_eflags().set_interrupt_flag(true)),
            // Process::new(diyng_process, Eflags::get_eflags().set_interrupt_flag(true)),
            // Process::new(fork_process, Eflags::get_eflags().set_interrupt_flag(true)),
            // Process::new(fork_bomb, Eflags::get_eflags().set_interrupt_flag(true)),
            // Process::new(fork_test_different_stack, Eflags::get_eflags().set_interrupt_flag(true)),
        ];
        let all_process = {
            let mut a = HashMap::new();
            for p in test_process.into_iter() {
                a.insert(p.pid, p);
            }
            a
        };
        Self {
            running_process: all_process.keys().map(|x| *x).collect(),
            all_process,
            curr_process_index: 0,
            no_process: true,
        }
    }

    /// get current process mutably
    fn curr_process_mut(&mut self) -> &mut Process {
        self.all_process.get_mut(&self.running_process[self.curr_process_index]).unwrap()
    }

    #[allow(dead_code)]
    /// get current process
    fn curr_process(&self) -> &Process {
        self.all_process.get(&self.running_process[self.curr_process_index]).unwrap()
    }

    /// save in the current process the cpu_state after an interruption
    pub fn save_process_state(&mut self, cpu_state: CpuState) {
        // dbg_hex!(cpu_state);
        if self.no_process {
            self.no_process = false;
            return;
        }
        self.curr_process_mut().cpu_state = cpu_state;
    }

    /// return to the process after a syscall which has return value `return value`
    // pub fn return_from_syscall(&mut self, return_value: i32) -> ! {
    //     self.curr_process_mut().cpu_state.registers.eax = return_value as u32;
    //     self.return_to_process()
    // }

    // /// return at the execution of the current process
    // pub fn return_to_process(&self) -> ! {
    //     let next_process = self.curr_process();
    //     // eprintln!("{:X?}", &next_process);
    //     unsafe {
    //         next_process.virtual_allocator.context_switch();
    //         SCHEDULER.force_unlock();
    //         _switch_process(next_process.cpu_state);
    //     }
    // }

    #[allow(dead_code)]
    /// set current process to the next process in the list of running process
    fn switch_next_process(&mut self) {
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
    }

    // /// Perform a fork
    // pub fn fork(&mut self) -> i32 {
    //     let curr_process = self.curr_process_mut();

    //     match curr_process.fork() {
    //         Ok(child) => {
    //             let child_pid = child.pid;
    //             self.running_process.push(child_pid);
    //             self.all_process.insert(child_pid, child);
    //             child_pid as i32
    //         }
    //         Err(e) => {
    //             eprintln!("{:?}", e);
    //             -1
    //         }
    //     }
    // }

    // /// Perform the exit syscall
    // /// remove the process from the list of running process and schedule to an other process
    // pub fn exit(&mut self, status: i32) -> ! {
    //     self.curr_process_mut().exit(status);
    //     self.running_process.remove(self.curr_process_index);
    //     self.switch_next_process();
    //     self.return_to_process()
    // }
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
