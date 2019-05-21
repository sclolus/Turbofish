//! this file contains the scheduler description

use super::{CpuState, Process, TaskMode};

use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
use lazy_static::lazy_static;

use crate::drivers::PIT0;
use crate::spinlock::Spinlock;

extern "C" {
    static mut SCHEDULER_COUNTER: i32;
}

type Pid = u32;

/// State of a process
#[derive(Debug, Clone)]
enum ProcessState {
    // Terminated { status: i32 },
    Running,
    // Waiting,
}

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(cpu_state: *mut CpuState) -> u32 {
    let mut scheduler = SCHEDULER.lock();
    SCHEDULER_COUNTER = scheduler.time_interval.unwrap();
    scheduler.set_process_state(*cpu_state);
    scheduler.switch_next_process();
    *cpu_state = scheduler.get_process_state();
    cpu_state as u32
}

struct Item {
    #[allow(dead_code)]
    state: ProcessState,
    process: Process,
}

/// Scheduler structure
pub struct Scheduler {
    /// contains pids of all runing process
    running_process: Vec<Pid>,
    /// contains a hashmap of pid, process
    all_process: HashMap<Pid, Item>,
    /// index in the vector of the current running process
    curr_process_index: Option<usize>, // TODO: May be better if we use PID instead ?
    /// time interval in PIT tics between two schedules
    time_interval: Option<i32>,
}

/// Base Scheduler implementation
impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self { running_process: Vec::new(), all_process: HashMap::new(), curr_process_index: None, time_interval: None }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_process(&mut self, process: Process) -> Pid {
        let pid = get_available_pid();
        self.all_process.insert(pid, Item { state: ProcessState::Running, process });
        self.running_process.push(pid);
        pid
    }

    /// Initialize the first processs and get a pointer to it)
    fn init_process_zero(&mut self) -> &Process {
        // Check if we got some processes to launch
        assert!(self.all_process.len() != 0);

        self.curr_process_index = Some(0);
        &self.curr_process().process
    }

    /// Set in the current process the cpu_state
    fn set_process_state(&mut self, cpu_state: CpuState) {
        self.curr_process_mut().process.set_process_state(cpu_state)
    }

    /// Get in the current process the cpu_state
    fn get_process_state(&self) -> CpuState {
        self.curr_process().process.get_process_state()
    }

    /// Set current process to the next process in the list of running process
    fn switch_next_process(&mut self) {
        self.curr_process_index = Some((self.curr_process_index.unwrap() + 1) % self.running_process.len());
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
            Ok(child) => self.add_process(child) as i32,
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

    // Set the PIT divisor if multitasking is enable
    let t = match task_mode {
        TaskMode::Mono => {
            log::info!("Scheduler initialised at mono-task");
            (-1, None)
        }
        TaskMode::Multi(scheduler_frequency) => {
            log::info!("Scheduler initialised at frequency: {:?} hz", scheduler_frequency);
            let period = (PIT0.lock().get_frequency().unwrap() / scheduler_frequency) as i32;
            if period == 0 {
                (1, Some(1))
            } else {
                (period, Some(period))
            }
        }
    };
    SCHEDULER_COUNTER = t.0;
    let mut scheduler = SCHEDULER.lock();
    scheduler.time_interval = t.1;

    // Initialise the first process and get a reference on it
    let p = scheduler.init_process_zero();

    // force unlock the scheduler as process borrows it and we won't get out of scope
    SCHEDULER.force_unlock();
    // After futur IRET for final process creation, interrupt must be re-enabled
    p.launch()
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
