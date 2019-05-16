use crate::process::{CpuState, Process};
// use crate::registers::Eflags;
use crate::spinlock::Spinlock;
use crate::syscall::{_user_exit, _user_fork};
use alloc::vec;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
// use hashmap_core::map::HashMap;
use lazy_static::lazy_static;

extern "C" {
    /// set all processor state to its arguments and iret to eip
    fn _switch_process(cpu_state: CpuState) -> !;
    static mut SCHEDULER_ACTIVE: bool;
}

type Pid = u32;

/// the pit handler
#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(cpu_state: CpuState) -> ! {
    let mut scheduler = SCHEDULER.lock();
    scheduler.save_process_state(cpu_state);
    scheduler.switch_next_process();
    scheduler.return_to_process()
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
    // create a new scheduler for tests
    unsafe fn new() -> Self {
        let test_process = vec![
            Process::new(0 as *mut u8, 0),
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
    pub fn return_from_syscall(&mut self, return_value: i32) -> ! {
        self.curr_process_mut().cpu_state.registers.eax = return_value as u32;
        self.return_to_process()
    }

    /// return at the execution of the current process
    pub fn return_to_process(&self) -> ! {
        let next_process = self.curr_process();
        // eprintln!("{:X?}", &next_process);
        unsafe {
            next_process.virtual_allocator.context_switch();
            SCHEDULER.force_unlock();
            _switch_process(next_process.cpu_state);
        }
    }

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

pub fn init() {
    unsafe {
        SCHEDULER_ACTIVE = true;
    }
}

/// stupid kernel space process a
#[allow(dead_code)]
fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
        }
    }
}

/// stupid kernel space process b
#[allow(dead_code)]
fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
        }
    }
}

/// stupid kernel space process diying in pain
#[allow(dead_code)]
fn diyng_process() {
    unsafe {
        for i in 0..10 {
            user_eprintln!("process diying slowly {}", i);
        }
        _user_exit(0);
    }
}

/// stupid kernel space process doing a fork
#[allow(dead_code)]
fn fork_process() {
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            for i in 0..1000000 {
                user_eprintln!("i am a gentle child {}", i);
                asm!("hlt"::::"volatile");
            }
        } else {
            for i in 0..1000000 {
                user_eprintln!("i am a proud father of child with pid({}) {}", fork_res, i);
                asm!("hlt"::::"volatile");
            }
        }
        _user_exit(0);
    }
}

/// stupid kernel space process doing a fork
#[allow(dead_code)]
fn fork_test_different_stack() {
    let mut array: [u8; 128] = [42; 128];
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            user_eprintln!("i am a gentle child");
            user_eprintln!("i am a gentle child {:?}", &array[..]);
            array = [21; 128];
            asm!("hlt"::::"volatile");
            user_eprintln!("i am a gentle child {:?}", &array[..]);
        } else {
            user_eprintln!("i am a proud father of child with pid({})", fork_res);
            user_eprintln!("in the father {:?}", &array[..]);
            asm!("hlt"::::"volatile");
            array = [84; 128];
            user_eprintln!("in the father {:?}", &array[..]);
        }
        loop {}
    }
}

// /// stupid kernel space process doing a fork
// #[allow(dead_code)]
// fn fork_test_different_stack() {
//     let mut array: [u8; 128] = [42; 128];
//     unsafe {
//         user_eprintln!("i am a the fork process");

//         let fork_res = _user_fork();
//         if fork_res == 0 {
//             user_eprintln!("i am a gentle child");
//             user_eprintln!("{}", format!("i am a gentle child {:?}", &array[..]));
//             array = [21; 128];
//             asm!("hlt"::::"volatile");
//             user_eprintln!("{}", format!("i am a gentle child {:?}", &array[..]));
//         } else {
//             user_eprintln!("i am a proud father of child with pid({})", fork_res);
//             user_eprintln!("{}", format!("in the father {:?}", &array[..]));
//             asm!("hlt"::::"volatile");
//             array = [84; 128];
//             user_eprintln!("{}", format!("in the father {:?}", &array[..]));
//         }
//         loop {}
//     }
// }

#[allow(dead_code)]
#[allow(unconditional_recursion)]
fn fork_bomb() {
    unsafe {
        user_eprintln!("i am a the fork process");

        let fork_res = _user_fork();
        if fork_res == 0 {
            user_eprintln!("i am a gentle child ");
            fork_bomb()
        } else {
            user_eprintln!("i am a proud father of child with pid({})", fork_res);
            fork_bomb()
        }
    }
}
