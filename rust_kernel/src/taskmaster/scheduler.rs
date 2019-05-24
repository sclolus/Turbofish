//! this file contains the scheduler description

use super::{CpuState, Process, SysResult, TaskMode};

use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;

use crate::drivers::PIT0;
use spinlock::Spinlock;

extern "C" {
    static mut SCHEDULER_COUNTER: i32;
}

type Pid = u32;

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(cpu_state: *mut CpuState) -> u32 {
    let mut scheduler = SCHEDULER.lock();
    SCHEDULER_COUNTER = scheduler.time_interval.unwrap();
    scheduler.set_curr_process_state(*cpu_state);
    scheduler.switch_next_process();
    *cpu_state = scheduler.get_curr_process_state();
    cpu_state as u32
}

#[derive(Debug)]
enum ProcessState {
    /// The process is currently on running state
    Running(Process),
    /// The process is terminated and wait to deliver his testament to his father
    Zombie(i32),
}

#[derive(Debug)]
/// Scheduler structure
pub struct Scheduler {
    /// contains pids of all runing process
    running_process: Vec<Pid>,
    /// contains a hashmap of pid, process
    all_process: HashMap<Pid, ProcessState>,
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
        self.all_process.insert(pid, ProcessState::Running(process));
        self.running_process.push(pid);
        pid
    }

    /// Initialize the first processs and get a pointer to it)
    fn init_process_zero(&mut self) -> &Process {
        // Check if we got some processes to launch
        assert!(self.all_process.len() != 0);

        self.curr_process_index = Some(0);

        match self.curr_process() {
            ProcessState::Running(process) => &process,
            ProcessState::Zombie(_) => panic!("no running process"),
        }
    }

    /// Set in the current process the cpu_state
    fn set_curr_process_state(&mut self, cpu_state: CpuState) {
        match self.curr_process_mut() {
            ProcessState::Running(process) => process.set_process_state(cpu_state),
            ProcessState::Zombie(_) => panic!("Zombie have not process state"),
        }
    }

    /// Get in the current process the cpu_state
    fn get_curr_process_state(&self) -> CpuState {
        match self.curr_process() {
            ProcessState::Running(process) => process.get_process_state(),
            ProcessState::Zombie(_) => panic!("Zombie have not process state"),
        }
    }

    /// Set current process to the next process in the list of running process
    fn switch_next_process(&mut self) {
        self.curr_process_index = Some((self.curr_process_index.unwrap() + 1) % self.running_process.len());
        // Dont forget to switch the Page diectory to the next process
        unsafe {
            match self.curr_process() {
                ProcessState::Running(process) => process.virtual_allocator.context_switch(),
                ProcessState::Zombie(_) => panic!("Zombie have not page directory"),
            };
        }
    }

    /// Get current process
    fn curr_process(&self) -> &ProcessState {
        self.all_process.get(&self.running_process[self.curr_process_index.unwrap()]).unwrap()
    }

    /// Get current process mutably
    fn curr_process_mut(&mut self) -> &mut ProcessState {
        self.all_process.get_mut(&self.running_process[self.curr_process_index.unwrap()]).unwrap()
    }

    /// Perform a fork
    pub fn fork(&mut self, cpu_state: CpuState) -> SysResult<i32> {
        let curr_process = match self.curr_process_mut() {
            ProcessState::Running(process) => process,
            ProcessState::Zombie(_) => panic!("Zombie cannot be forked"),
        };
        curr_process.fork(cpu_state).map(|child| self.add_process(child) as i32)
    }

    // TODO: Send a status signal to the father
    // TODO: fflush process ressources
    // TODO: Remove completely process from scheduler after death attestation
    /// Exit form a process and change the current process
    pub fn exit(&mut self, status: i32, cpu_state: *mut CpuState) -> SysResult<i32> {
        eprintln!(
            "exit called for process with PID: {:?} STATUS: {:?}",
            self.running_process[self.curr_process_index.unwrap()],
            status
        );
        // Modifie the status of the process to zombie with status (drop process implicitely)
        let pid = self.running_process[self.curr_process_index.unwrap()];
        self.all_process.insert(pid, ProcessState::Zombie(status));
        // Remove process from the running process list
        self.running_process.remove(self.curr_process_index.unwrap());
        // Check if there is altmost one process
        if self.running_process.len() == 0 {
            eprintln!("no more process !");
            loop {}
        }
        self.curr_process_index = Some(self.curr_process_index.unwrap() % self.running_process.len());
        // Switch to the next process
        unsafe {
            SCHEDULER_COUNTER = self.time_interval.unwrap();

            match self.curr_process() {
                ProcessState::Running(process) => process.virtual_allocator.context_switch(),
                ProcessState::Zombie(_) => panic!("Zombie have not page directory"),
            };

            *cpu_state = self.get_curr_process_state();
            // Don't modify EAX of the current process (syscall ret)
            Ok((*cpu_state).registers.eax as i32)
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

    println!("Starting processes:");

    // After futur IRET for final process creation, interrupt must be re-enabled
    p.start()
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
