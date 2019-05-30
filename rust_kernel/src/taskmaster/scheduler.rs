//! this file contains the scheduler description

use super::{Process, SysResult, TaskMode};

use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;

use alloc::collections::CollectionAllocErr;

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
    // let cpu_state: *const super::CpuState = kernel_esp as *const super::CpuState;
    // if (*cpu_state).cs == 0x08 {
    //     eprintln!("Syscall interrupted for process_idx: {:?} !", scheduler.curr_process_index);
    // }
    SCHEDULER_COUNTER = scheduler.time_interval.unwrap();

    // Backup of the current process kernel_esp
    scheduler.curr_process_mut().unwrap_running_mut().kernel_esp = kernel_esp;
    // Switch between processes
    scheduler.advance_next_process();

    let p = scheduler.curr_process().unwrap_running();
    p.context_switch();
    // Restore kernel_esp for the new process
    p.kernel_esp
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32) {
    SCHEDULER.force_unlock();

    SCHEDULER.lock().all_process.get_mut(&process_to_free).unwrap().process_state = ProcessState::Zombie(status);
}

#[derive(Debug)]
struct Task {
    process_state: ProcessState,
    child: Vec<Pid>,
    parent: Option<Pid>,
}

impl Task {
    pub fn new(parent: Option<Pid>, process_state: ProcessState) -> Self {
        Self { process_state, child: Vec::new(), parent }
    }
    pub fn unwrap_running_mut(&mut self) -> &mut Process {
        match &mut self.process_state {
            ProcessState::Running(process) => process,
            ProcessState::Zombie(_) => panic!("WTF"),
        }
    }
    pub fn unwrap_running(&self) -> &Process {
        match &self.process_state {
            ProcessState::Running(process) => process,
            ProcessState::Zombie(_) => panic!("WTF"),
        }
    }
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
    all_process: HashMap<Pid, Task>,
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
    pub fn add_process(&mut self, father_pid: Option<Pid>, process: Process) -> Result<Pid, CollectionAllocErr> {
        let pid = get_available_pid();
        self.all_process.try_reserve(1)?;
        self.running_process.try_reserve(1)?;
        self.all_process.insert(pid, Task::new(father_pid, ProcessState::Running(process)));
        self.running_process.insert(self.curr_process_index, pid);
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
        Ok(pid)
    }

    /// Advance to the next process
    fn advance_next_process(&mut self) {
        self.curr_process_index = (self.curr_process_index + 1) % self.running_process.len();
    }

    /// Get current process
    fn curr_process(&self) -> &Task {
        self.all_process.get(&self.running_process[self.curr_process_index]).unwrap()
    }

    /// Get current process mutably
    fn curr_process_mut(&mut self) -> &mut Task {
        self.all_process.get_mut(&self.running_process[self.curr_process_index]).unwrap()
    }

    /// Get the current running process (usefull for syscalls)
    pub fn get_current_running_process(&mut self) -> &mut Process {
        // Check if we got some processes to launch
        assert!(self.all_process.len() != 0);

        self.curr_process_mut().unwrap_running_mut()
    }

    /// Get the current process PID
    fn curr_process_pid(&self) -> Pid {
        self.running_process[self.curr_process_index]
    }

    /// Perform a fork
    pub fn fork(&mut self, kernel_esp: u32) -> SysResult<i32> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        let father_pid = self.curr_process_pid();
        let curr_process = self.curr_process_mut();

        // try reserve a place for child pid
        curr_process.child.try_reserve(1)?;
        let child = curr_process.unwrap_running().fork(kernel_esp)?;
        let child_pid = self.add_process(Some(father_pid), child)?;

        self.curr_process_mut().child.push(child_pid);
        // dbg!(self.curr_process());

        Ok(child_pid as i32)
    }
    // TODO: Send a status signal to the father
    // TODO: Remove completely process from scheduler after death attestation
    /// Exit form a process and go to the current process
    pub fn exit(&mut self, status: i32) -> ! {
        // eprintln!("exiting {:?}", self.curr_process());
        // eprintln!(
        //     "exit called for process with PID: {:?} STATUS: {:?}",
        //     self.running_process[self.curr_process_index], status
        // );
        // Get the current process's PID
        let pid = self.curr_process_pid();

        // Remove process from the running process list
        self.running_process.remove(self.curr_process_index);
        // Check if there is altmost one process
        if self.running_process.len() == 0 {
            eprintln!("no more process !");
            loop {}
        }
        // } else {
        //     eprintln!("Stay {:?} processes in game", self.running_process.len());
        // }
        self.curr_process_index = self.curr_process_index % self.running_process.len();
        // Switch to the next process
        unsafe {
            SCHEDULER_COUNTER = self.time_interval.unwrap();

            let p = self.curr_process().unwrap_running();
            p.context_switch();

            _exit_resume(p.kernel_esp, pid, status);
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
    let p = scheduler.get_current_running_process();

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
