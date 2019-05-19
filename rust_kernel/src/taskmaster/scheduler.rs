//! this file contains the scheduler description

use super::{CpuState, Process, ProcessType, TaskMode};

use alloc::boxed::Box;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
use lazy_static::lazy_static;

use crate::spinlock::Spinlock;

extern "C" {
    static mut SCHEDULER_ACTIVE: bool;
}

type Pid = u32;

/// State of a process
#[derive(Debug, Clone)]
enum ProcessState {
    // Terminated { status: i32 },
    Running,
    // Waiting,
}

/// the pit handler
#[no_mangle]
extern "C" fn scheduler_interrupt_handler(cpu_state: *mut CpuState) {
    let mut scheduler = SCHEDULER.lock();
    scheduler.set_process_state(cpu_state);
    scheduler.switch_next_process();
    unsafe {
        *cpu_state = *scheduler.get_process_state();
    }
}

struct Item {
    #[allow(dead_code)]
    state: ProcessState,
    process: Box<Process>,
}

/// Scheduler structure
pub struct Scheduler {
    /// contains pids of all runing process
    running_process: Vec<Pid>,
    /// contains a hashmap of pid, process
    all_process: HashMap<Pid, Item>,
    /// index in the vector of the current running process
    curr_process_index: Option<usize>, // TODO: May be better if we use PID instead ?
}

/// Base Scheduler implementation
impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self { running_process: Vec::new(), all_process: HashMap::new(), curr_process_index: None }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_process(&mut self, process: Box<Process>) {
        let pid = get_available_pid();
        self.all_process.insert(pid, Item { state: ProcessState::Running, process });
        self.running_process.push(pid);
    }

    /// Initialize the first processs and get a pointer to it)
    fn init_process_zero(&mut self) -> *const Box<Process> {
        // Check if we got some processes to launch
        assert!(self.all_process.len() != 0);

        self.curr_process_index = Some(0);
        let p = self.curr_process_mut();
        // TODO: Manage kernel process
        assert!(p.process.process_type != ProcessType::Kernel);
        &p.process
    }

    /// Set in the current process the cpu_state
    fn set_process_state(&mut self, cpu_state: *const CpuState) {
        self.curr_process_mut().process.set_process_state(cpu_state)
    }

    /// Get in the current process the cpu_state
    fn get_process_state(&mut self) -> *const CpuState {
        self.curr_process_mut().process.get_process_state()
    }

    /// Set current process to the next process in the list of running process
    fn switch_next_process(&mut self) {
        self.curr_process_index = Some((self.curr_process_index.unwrap() + 1) % self.running_process.len());
        // TODO: Manage kernel process
        assert!(self.curr_process().process.process_type != ProcessType::Kernel);
        // Dont forget to switch the Page diectory to the next process
        unsafe {
            self.curr_process().process.virtual_allocator.context_switch();
        }
    }

    /// Get current process
    fn curr_process(&self) -> &Item {
        self.all_process.get(&self.running_process[self.curr_process_index.unwrap()]).unwrap()
    }

    /// Get current process mutably
    fn curr_process_mut(&mut self) -> &mut Item {
        self.all_process.get_mut(&self.running_process[self.curr_process_index.unwrap()]).unwrap()
    }

    /// Perform a fork
    #[allow(dead_code)]
    pub fn fork(&mut self) -> i32 {
        let curr_process = self.curr_process_mut();

        match curr_process.process.fork() {
            Ok(child) => {
                let child_pid = get_available_pid();
                self.running_process.push(child_pid);
                self.all_process.insert(child_pid, Item { state: ProcessState::Running, process: child });
                child_pid as i32
            }
            Err(e) => {
                eprintln!("{:?}", e);
                -1
            }
        }
    }
}

/// Start the whole scheduler
pub unsafe fn start(task_mode: TaskMode) -> ! {
    // Inhibit all hardware interrupts, particulary timer.
    asm!("cli");

    // Mark the scheduler as active if multitasking is enable
    SCHEDULER_ACTIVE = match task_mode {
        TaskMode::Mono => false,
        TaskMode::Multi => true,
    };

    // Initialise the first process and get a reference on it
    let p = SCHEDULER.lock().init_process_zero();

    // After futur IRET for final process creation, interrupt must be re-enabled
    (*p).launch();

    panic!("Unreachable");
}

lazy_static! {
    pub static ref SCHEDULER: Spinlock<Scheduler> = Spinlock::new(Scheduler::new());
}

use core::sync::atomic::{AtomicU32, Ordering};

/// represent the greatest available pid
static MAX_PID: AtomicU32 = AtomicU32::new(0);

/// get the next available pid for a new process
fn get_available_pid() -> Pid {
    MAX_PID.fetch_add(1, Ordering::Relaxed) // TODO: handle when overflow to 0
}

// /// Perform the exit syscall (TODO: NEED TO BE REIMPLEMENTED)
// /// remove the process from the list of running process and schedule to an other process
// pub fn exit(&mut self, status: i32) -> ! {
//     self.curr_process_mut().exit(status);
//     self.running_process.remove(self.curr_process_index);
//     self.switch_next_process(); I THINK IT IS BETTER TO PROGRAM THE DEAD AND WAIT THE NEXT SCHEDULE TICK
//     self.return_to_process() WTF
// }

// pub fn exit(&mut self, status: i32) (HERITANCE FROM PROCESS)
//     self.state = State::Terminated { status };
//     TODO: free resource
// }
