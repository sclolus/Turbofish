//! this file contains the scheduler description

use super::{CpuState, Process, SysResult, TaskMode};

use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;

use crate::drivers::PIT0;
use spinlock::Spinlock;

extern "C" {
    static mut SCHEDULER_COUNTER: i32;

    fn _exit_resume(new_kernel_esp: u32, process_to_free: Pid, status: i32) -> !;
}

type Pid = u32;

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(kernel_esp: u32) -> u32 {
    let mut scheduler = SCHEDULER.lock();

    let cpu_state: *const CpuState = kernel_esp as *const CpuState;
    if (*cpu_state).cs == 0x08 {
        eprintln!("Syscall interrupted for process_idx: {:?} !", scheduler.curr_process_index);
    }
    SCHEDULER_COUNTER = scheduler.time_interval.unwrap();

    // Backup of the current process kernel_esp
    match scheduler.curr_process_mut() {
        ProcessState::Running(process) => process.kernel_esp = kernel_esp,
        ProcessState::Zombie(_) => panic!("WTF"),
    };

    // Switch between processes
    scheduler.switch_next_process();

    // Restore kernel_esp for the new process
    match scheduler.curr_process() {
        ProcessState::Running(process) => process.kernel_esp,
        ProcessState::Zombie(_) => panic!("WTF"),
    }
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32) {
    SCHEDULER.force_unlock();

    SCHEDULER.lock().all_process.insert(process_to_free, ProcessState::Zombie(status));
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
    curr_process_index: usize, // TODO: May be better if we use PID instead ?
    /// time interval in PIT tics between two schedules
    time_interval: Option<i32>,
}

/// Base Scheduler implementation
impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self { running_process: Vec::new(), all_process: HashMap::new(), curr_process_index: 0, time_interval: None }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_process(&mut self, process: Process) -> Pid {
        let pid = get_available_pid();
        self.all_process.insert(pid, ProcessState::Running(process));
        self.running_process.insert(self.curr_process_index, pid);
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
        pid
    }

    /// Initialize the first processs and get a pointer to it)
    fn init_process_zero(&mut self) -> &Process {
        // Check if we got some processes to launch
        assert!(self.all_process.len() != 0);

        match self.curr_process() {
            ProcessState::Running(process) => &process,
            ProcessState::Zombie(_) => panic!("no running process"),
        }
    }

    /// Set current process to the next process in the list of running process
    fn switch_next_process(&mut self) {
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
        // Dont forget to switch the Page diectory to the next process
        unsafe {
            match self.curr_process() {
                ProcessState::Running(process) => {
                    // Switch to the new process PD
                    process.virtual_allocator.context_switch();
                    // Re-init the TSS block for the new process
                    process.init_tss();
                }
                ProcessState::Zombie(_) => panic!("Zombie have not page directory"),
            };
        }
    }

    /// Get current process
    fn curr_process(&self) -> &ProcessState {
        self.all_process.get(&self.running_process[self.curr_process_index]).unwrap()
    }

    /// Get current process mutably
    fn curr_process_mut(&mut self) -> &mut ProcessState {
        self.all_process.get_mut(&self.running_process[self.curr_process_index]).unwrap()
    }

    /// Perform a fork
    pub fn fork(&mut self, kernel_esp: u32) -> SysResult<i32> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        let curr_process = match self.curr_process_mut() {
            ProcessState::Running(process) => process,
            ProcessState::Zombie(_) => panic!("Zombie cannot be forked"),
        };
        Ok(curr_process.fork(kernel_esp).map(|child| self.add_process(child))? as i32)
    }

    // TODO: Send a status signal to the father
    // TODO: Remove completely process from scheduler after death attestation
    /// Exit form a process and go to the current process
    pub fn exit(&mut self, status: i32) -> ! {
        eprintln!(
            "exit called for process with PID: {:?} STATUS: {:?}",
            self.running_process[self.curr_process_index], status
        );
        // Get the current process's PID
        let pid = self.running_process[self.curr_process_index];

        // Flush the process's virtual allocator (until we are in his CR3)
        match self.curr_process_mut() {
            ProcessState::Running(process) => process.free_user_ressources(),
            ProcessState::Zombie(_) => panic!("WTF"),
        };

        // Remove process from the running process list
        self.running_process.remove(self.curr_process_index);
        // Check if there is altmost one process
        if self.running_process.len() == 0 {
            eprintln!("no more process !");
            loop {}
        } else {
            eprintln!("Stay {:?} processes in game", self.running_process.len());
        }
        self.curr_process_index = self.curr_process_index % self.running_process.len();
        // Switch to the next process
        unsafe {
            SCHEDULER_COUNTER = self.time_interval.unwrap();

            match self.curr_process() {
                ProcessState::Running(process) => {
                    // Switch to the new process PD
                    process.virtual_allocator.context_switch();
                    // Re-init the TSS block for the new process
                    process.init_tss();
                    // Follow the kernel stack of the new process
                    _exit_resume(process.kernel_esp, pid, status);
                }
                ProcessState::Zombie(_) => panic!("WTF"),
            };
        };
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
