//! this file contains the scheduler description

use super::process;
use super::process::{KernelProcess, Process, UserProcess};
use super::{SysResult, TaskMode};

pub mod task;
pub use task::{ProcessState, Task, WaitingState};

use alloc::boxed::Box;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;

use alloc::collections::CollectionAllocErr;
use fallible_collections::FallibleVec;

use errno::Errno;

use crate::drivers::PIT0;
use spinlock::Spinlock;

use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use core::ffi::c_void;

extern "C" {
    fn _exit_resume(new_kernel_esp: u32, process_to_free: Pid, status: i32) -> !;

    pub fn _get_pit_time() -> u32;
    pub fn _get_process_end_time() -> u32;

    fn _update_process_end_time(update: u32);

    pub fn _uninterruptible();
    pub fn _interruptible();
    pub fn _schedule_force_preempt();
}

pub type Pid = u32;

#[inline(always)]
pub fn uninterruptible() {
    unsafe {
        crate::taskmaster::scheduler::_uninterruptible();
    }
}

#[inline(always)]
pub fn interruptible() {
    unsafe {
        // Check if the Time to live of the current process is expired
        // TODO: If scheduler is disable, the kernel will crash
        // TODO: After Exit, the next process seems to be skiped !
        if crate::taskmaster::scheduler::_get_pit_time() >= crate::taskmaster::scheduler::_get_process_end_time() {
            asm!("int 0x81" :::: "intel", "volatile");
        } else {
            crate::taskmaster::scheduler::_interruptible();
        }
    }
}

pub fn auto_preempt() {
    unsafe {
        SCHEDULER.force_unlock();
        asm!("int 0x81" :::: "volatile","intel");
    }
}

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(kernel_esp: u32) -> u32 {
    let mut scheduler = SCHEDULER.lock();
    // let cpu_state: *const super::CpuState = kernel_esp as *const super::CpuState;
    // if (*cpu_state).cs == 0x08 {
    //     eprintln!("Syscall interrupted for process_idx: {:?} !", scheduler.curr_process_index);
    // }
    _update_process_end_time(scheduler.time_interval.unwrap());

    // Store the current kernel stack pointer
    scheduler.store_kernel_esp(kernel_esp);

    // Switch between processes
    scheduler.advance_next_process(1);

    // Set all the context of the illigible process
    let new_kernel_esp = scheduler.load_new_context();

    if scheduler.idle_mode == false {
        let p = scheduler.curr_process_mut();
        p.check_pending_signals();
    }

    // Restore kernel_esp for the new process/
    new_kernel_esp
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32) {
    SCHEDULER.force_unlock();

    SCHEDULER.lock().all_process.get_mut(&process_to_free).unwrap().process_state = ProcessState::Zombie(status);
    interruptible();
}

#[derive(Debug)]
/// Scheduler structure
pub struct Scheduler {
    /// contains pids of all runing process
    running_process: Vec<Pid>,
    /// contains a hashmap of pid, process
    all_process: HashMap<Pid, Task>,
    /// index in the vector of the current running process
    curr_process_pid: Pid,
    /// current process index in the running_process vector
    curr_process_index: usize,
    /// time interval in PIT tics between two schedules
    time_interval: Option<u32>,
    /// The scheduler must have an idle kernel proces if all the user process are waiting
    kernel_idle_process: Option<Box<KernelProcess>>,
    /// Indicate if the scheduler is on idle mode. TODO: Use the boolinator xD
    idle_mode: bool,
}

/// Base Scheduler implementation
impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            running_process: Vec::new(),
            all_process: HashMap::new(),
            curr_process_index: 0,
            curr_process_pid: 0,
            time_interval: None,
            kernel_idle_process: None,
            idle_mode: false,
        }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_user_process(
        &mut self,
        father_pid: Option<Pid>,
        process: Box<UserProcess>,
    ) -> Result<Pid, CollectionAllocErr> {
        let pid = get_available_pid();
        self.all_process.try_reserve(1)?;
        self.running_process.try_reserve(1)?;
        self.all_process.insert(pid, Task::new(father_pid, ProcessState::Running(process)));
        self.running_process.push(pid);
        Ok(pid)
    }

    /// Set the idle process for the scheduler
    pub fn set_idle_process(&mut self, idle_process: Box<KernelProcess>) -> Result<(), ()> {
        self.kernel_idle_process = Some(idle_process);
        Ok(())
    }

    /// Backup of the current process kernel_esp
    fn store_kernel_esp(&mut self, kernel_esp: u32) {
        match self.idle_mode {
            true => {
                self.kernel_idle_process.as_mut().expect("No idle mode process").kernel_esp = kernel_esp;
                self.idle_mode = false;
            }
            false => {
                self.curr_process_mut().unwrap_running_mut().kernel_esp = kernel_esp;
            }
        }
    }

    /// Advance until a next elligible process was found
    fn advance_next_process(&mut self, offset: usize) {
        let next_process_index = (self.curr_process_index + offset) % self.running_process.len();

        for idx in next_process_index..next_process_index + self.running_process.len() {
            self.curr_process_index = idx % self.running_process.len();
            self.curr_process_pid = self.running_process[self.curr_process_index];

            match &self.curr_process().process_state {
                ProcessState::Running(_) | ProcessState::Signaled(_) => return,
                ProcessState::Waiting(_, waiting_state) => match waiting_state {
                    WaitingState::Sleeping(time) => unsafe {
                        let now = _get_pit_time();
                        if now >= *time {
                            self.curr_process_mut().set_running();
                            return;
                        }
                    },
                    WaitingState::ChildDeath(_) => {}
                },
                ProcessState::Zombie(_) => panic!("WTF"),
            };
        }
        self.idle_mode = true;
    }

    /// Prepare the context for the new illigible process
    fn load_new_context(&mut self) -> u32 {
        match self.idle_mode {
            true => {
                let process = self.kernel_idle_process.as_ref();
                process.expect("No idle mode process").kernel_esp
            }
            false => {
                let process = self.curr_process().unwrap_running();
                unsafe {
                    process.context_switch();
                }
                process.kernel_esp
            }
        }
    }

    /// Get current process pid
    pub fn curr_process_pid(&self) -> Pid {
        self.curr_process_pid
    }

    /// Get current process
    fn curr_process(&self) -> &Task {
        self.get_process(self.curr_process_pid).unwrap()
    }

    /// Get current process mutably
    pub fn curr_process_mut(&mut self) -> &mut Task {
        self.get_process_mut(self.curr_process_pid).unwrap()
    }

    pub fn get_process(&self, pid: Pid) -> Option<&Task> {
        self.all_process.get(&pid)
    }

    pub fn get_process_mut(&mut self, pid: Pid) -> Option<&mut Task> {
        self.all_process.get_mut(&pid)
    }

    /// Remove the current running process
    fn remove_curr_running(&mut self) {
        // Remove process from the running process list
        self.running_process.remove(self.curr_process_index);
        // Check if there is altmost one process
        if self.running_process.len() == 0 {
            eprintln!("no more process !");
            loop {}
        }
    }

    /// Perform a fork
    pub fn fork(&mut self, kernel_esp: u32) -> SysResult<u32> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        let father_pid = self.curr_process_pid;
        let curr_process = self.curr_process_mut();

        // try reserve a place for child pid
        curr_process.child.try_reserve(1)?;
        let child = curr_process.unwrap_running().fork(kernel_esp)?;
        let child_pid = self.add_user_process(Some(father_pid), child)?;

        self.curr_process_mut().child.push(child_pid);
        // dbg!(self.curr_process());

        Ok(child_pid)
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
        let p = self.curr_process();

        if let Some(father_pid) = p.parent {
            let father = self.all_process.get_mut(&father_pid).expect("process parent should exist");
            if father.is_waiting() {
                self.running_process.try_push(father_pid).unwrap();
                // dbg!("exit father set running");
                father.set_running();
            }
        }
        let pid = self.curr_process_pid;

        self.remove_curr_running();

        self.advance_next_process(0);

        // Switch to the next process
        unsafe {
            _update_process_end_time(self.time_interval.unwrap());

            let new_kernel_esp = self.load_new_context();

            _exit_resume(new_kernel_esp, pid, status);
        };
    }

    // TODO: Solve Gloubiboulga
    pub fn wait(&mut self) -> SysResult<Pid> {
        let mut p = self.all_process.remove(&self.curr_process_pid).unwrap();
        if p.child.is_empty() {
            return Err(Errno::Echild);
        }
        // TODO: Solve Borrow
        if let None = p.child.iter().find(|c| self.all_process.get(c).unwrap().is_zombie()) {
            p.set_waiting(WaitingState::ChildDeath(None));
            // dbg!("set waiting");
            self.all_process.insert(self.curr_process_pid, p);
            self.remove_curr_running();

            auto_preempt();
            // dbg!("return to live after schedule");
        }
        // if let Some(child) = p.child.iter().find(|c| self.all_process.get(c).unwrap().is_zombie()) {
        //     child.exit_status
        // }
        Ok(0)
    }
}

/// Start the whole scheduler
pub unsafe fn start(task_mode: TaskMode) -> ! {
    // Inhibit all hardware interrupts, particulary timer.
    asm!("cli" :::: "volatile");

    // Register a new IDT entry in 81h for force preempting
    let mut interrupt_table = InterruptTable::current_interrupt_table().unwrap();

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(0)
        .set_selector(1 << 3)
        .set_gate_type(InterruptGate32);
    gate_entry.set_handler(_schedule_force_preempt as *const c_void as u32);
    interrupt_table[0x81] = gate_entry;

    // Set the PIT divisor if multitasking is enable
    let t = match task_mode {
        TaskMode::Mono => {
            log::info!("Scheduler initialised at mono-task");
            None
        }
        TaskMode::Multi(scheduler_frequency) => {
            log::info!("Scheduler initialised at frequency: {:?} hz", scheduler_frequency);
            let period = (PIT0.lock().get_frequency().unwrap() / scheduler_frequency) as u32;
            if period == 0 {
                Some(1)
            } else {
                Some(period)
            }
        }
    };
    let mut scheduler = SCHEDULER.lock();
    scheduler.time_interval = t;

    // Initialise the first process and get a reference on it
    let p = scheduler.curr_process_mut().unwrap_running_mut();

    // force unlock the scheduler as process borrows it and we won't get out of scope
    SCHEDULER.force_unlock();

    println!("Starting processes:");

    match t {
        Some(v) => _update_process_end_time(v),
        None => _update_process_end_time(-1 as i32 as u32),
    }

    interruptible();
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
