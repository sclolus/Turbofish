//! this file contains the scheduler description

use super::process::{KernelProcess, Process, UserProcess};
use super::task::{ProcessState, Task, WaitingState};
use super::{SysResult, TaskMode};

use alloc::boxed::Box;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};
use hashmap_core::fnv::FnvHashMap as HashMap;
use spinlock::Spinlock;

use crate::drivers::PIT0;
use crate::interrupts;
use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};

extern "C" {
    fn _exit_resume(new_kernel_esp: u32, process_to_free: Pid, status: i32) -> !;

    fn _auto_preempt() -> i32;

    pub fn _get_pit_time() -> u32;
    pub fn _get_process_end_time() -> u32;

    fn _update_process_end_time(update: u32);

    pub fn _unpreemptible();
    pub fn _preemptible();
    pub fn _schedule_force_preempt();
}

pub type Pid = u32;

/// Protect process again scheduler interruption
#[inline(always)]
pub fn unpreemptible() {
    unsafe {
        crate::taskmaster::scheduler::_unpreemptible();
    }
}

/// Allow scheduler to interrupt process execution
#[inline(always)]
pub fn preemptible() {
    unsafe {
        if INTERRUPTIBLE_LOCK == false {
            // Check if the Time to live of the current process is expired
            // TODO: If scheduler is disable, the kernel will crash
            // TODO: After Exit, the next process seems to be skiped !
            if crate::taskmaster::scheduler::_get_pit_time() >= crate::taskmaster::scheduler::_get_process_end_time() {
                _auto_preempt();
            } else {
                crate::taskmaster::scheduler::_preemptible();
            }
        }
    }
}

/// A Finalizer-pattern Struct that disables preemption upon instantiation.
/// then reenables it at Drop time.
pub struct PreemptionGuard;

impl PreemptionGuard {
    /// The instantiation methods that disables preemption and creates the guard.
    pub fn new() -> Self {
        unpreemptible();
        Self
    }
}

impl Drop for PreemptionGuard {
    /// The drop implementation of the guard reenables preemption.
    fn drop(&mut self) {
        preemptible();
    }
}

#[macro_export]
/// This macro executes the block given as parameter in an unpreemptible context.
macro_rules! unpreemptible_context {
    ($code: block) => {{
        /// You probably shouldn't use it outside of taskmaster, but we never know.
        /// The absolute path is used not to fuck up the compilation if the parent module
        /// does not have the module scheduler as submodule.
        use crate::taskmaster::scheduler::PreemptionGuard;

        let _guard = PreemptionGuard::new();

        $code
    }};
}

/// Inhibit the effect of the interruptible() function
/// For safety reasons, this function must be used only in non-interruptible INTGATE
#[inline(always)]
pub fn lock_interruptible() {
    unsafe {
        INTERRUPTIBLE_LOCK = true;
    }
}

/// Allow again the effect of the interruptible() function
/// For safety reason, this function must be used only if the lock is TRUE
#[inline(always)]
pub fn unlock_interruptible() {
    unsafe {
        INTERRUPTIBLE_LOCK = false;
    }
}

static mut INTERRUPTIBLE_LOCK: bool = false;

pub fn auto_preempt() -> i32 {
    unsafe {
        SCHEDULER.force_unlock();
        _auto_preempt()
    }
}

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(kernel_esp: u32) -> u32 {
    let mut scheduler = SCHEDULER.lock();
    _update_process_end_time(scheduler.time_interval.unwrap());

    // Store the current kernel stack pointer
    scheduler.store_kernel_esp(kernel_esp);

    // Switch between processes
    // let signal = scheduler.advance_next_process(1);
    scheduler.advance_next_process(1);

    // Set all the context of the illigible process
    let new_kernel_esp = scheduler.load_new_context();

    // After loading the new context:
    // If ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
    // If ring0 process -> block temporary interruptible macro

    // if let Some(HANDLED(_) = signal {
    //     GET_RING(new_kernel_esp) == 3 {
    //         APPLY_PENDING_SIGNAL(new_kernel_esp);
    //     } else {
    //         lock_interruptible();
    //     }
    // }

    // Restore kernel_esp for the new process/
    new_kernel_esp
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32) {
    SCHEDULER.force_unlock();

    SCHEDULER.lock().all_process.get_mut(&process_to_free).unwrap().process_state = ProcessState::Zombie(status);

    preemptible();
}

#[derive(Debug)]
/// Scheduler structure
pub struct Scheduler {
    /// contains a hashmap of pid, process
    pub all_process: HashMap<Pid, Task>,
    /// contains pids of all runing process
    running_process: Vec<Pid>,

    /// The next pid to be considered by the scheduler
    /// TODO: think about PID Reuse when SMP will be added,
    /// as current PID attribution depends on the existence of a pid in the
    /// `all_process` HashMap.
    next_pid: AtomicU32,

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
            next_pid: AtomicU32::new(1),
            curr_process_index: 0,
            curr_process_pid: 1,
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
        let pid = self.get_available_pid();
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
    // Must return signal option: Option<enum SIGTYPE>
    // fn advance_next_process(&mut self, offset: usize) -> Option<enum SIGTYPE> {...}
    fn advance_next_process(&mut self, offset: usize) {
        let next_process_index = (self.curr_process_index + offset) % self.running_process.len();

        for idx in next_process_index..next_process_index + self.running_process.len() {
            self.curr_process_index = idx % self.running_process.len();
            self.curr_process_pid = self.running_process[self.curr_process_index];

            // Check if pending signal: Signal Can interrupt all except zombie
            // some signals may be marked as IGNORED, Remove signal and dont DO anything in this case
            // Call immediately exit with status if DEADLY_SIGNAL (no handled OR unblockable)
            // else create a signal var with option<Signum>

            // let signal = CHECK_PENDING_SIGNAL() -> Option<enum SIGTYPE> {
            // None, no signal or ignored internal
            // Some(DEADLY(SIGNUM)) -> Internal call to exit(SIGNUM)
            // Some(HANDLED(SIGNUM)) ->  continue...
            // }
            match &self.curr_process().process_state {
                ProcessState::Running(_) => return,
                ProcessState::Waiting(_, waiting_state) => match waiting_state {
                    WaitingState::Sleeping(time) => unsafe {
                        // Check if signal var contains something, set return value as negative (rel to SIGNUM), set process as running then return

                        // if let Some(HANDLED(SIGNUM)) = signal {
                        //     self.curr_process_mut().set_running();
                        //     self.curr_process_mut().set_return_value(-SIGNUM)
                        //     return signal;
                        // }

                        let now = _get_pit_time();
                        if now >= *time {
                            self.curr_process_mut().set_running();
                            self.curr_process_mut().set_return_value(0);
                            return;
                        }
                    },
                    WaitingState::ChildDeath(pid_opt, _) => {
                        // Check if signal var contains something, set return value as negative (rel to SIGNUM), set process as running then return

                        // if let Some(HANDLED(SIGNUM)) = signal {
                        //     self.curr_process_mut().set_running();
                        //     self.curr_process_mut().set_return_value(-SIGNUM)
                        //     return signal;
                        // }

                        let zombie_pid = match pid_opt {
                            // In case of PID == None, Check is the at least one child is a zombie.
                            None => {
                                if let Some(&zombie_pid) = self.curr_process().child.iter().find(|current_pid| {
                                    self.all_process.get(current_pid).expect("Hashmap corrupted").is_zombie()
                                }) {
                                    Some(zombie_pid)
                                } else {
                                    None
                                }
                            }
                            // In case of PID >= 0, Check is specified child PID is a zombie.
                            Some(pid) => {
                                if let Some(elem) =
                                    self.curr_process().child.iter().find(|&&current_pid| current_pid == *pid as u32)
                                {
                                    if self.all_process.get(elem).expect("Hashmap corrupted").is_zombie() {
                                        Some(*elem)
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        };
                        // If a zombie was found, write the exit status, overwrite PID if None and return
                        if let Some(pid) = zombie_pid {
                            let child = self.all_process.get(&pid).expect("Hashmap corrupted");
                            match child.process_state {
                                ProcessState::Zombie(status) => {
                                    self.curr_process_mut()
                                        .set_waiting(WaitingState::ChildDeath(zombie_pid, status as u32));
                                    self.curr_process_mut().set_return_value(0);
                                    return;
                                }
                                _ => panic!("A zombie was found just before, but there is no zombie here"),
                            };
                        }
                    }
                },
                ProcessState::Zombie(_) => panic!("A zombie cannot be in the running list"),
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
                let p = self.curr_process_mut();

                let process = p.unwrap_running();
                unsafe {
                    process.context_switch();
                }
                let kernel_esp = process.kernel_esp;
                kernel_esp
            }
        }
    }

    /// Get current process pid
    pub fn curr_process_pid(&self) -> Pid {
        self.curr_process_pid
    }

    /// Get current process
    pub fn curr_process(&self) -> &Task {
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

    const REAPER_PID: Pid = 1;

    // TODO: Send a status signal to the father
    /// Exit form a process and go to the current process
    pub fn exit(&mut self, status: i32) -> ! {
        // println!(
        //     "exit called for process with PID: {:?} STATUS: {:?}",
        //     self.running_process[self.curr_process_index], status
        // );

        // When the father die, the process 0 adopts all his orphelans
        if let Some(reaper) = self.all_process.get(&Self::REAPER_PID) {
            if let ProcessState::Zombie(_) = reaper.process_state {
                eprintln!("... the reaper is a zombie ... it is worring ...");
            }
            while let Some(child_pid) = self.curr_process_mut().child.pop() {
                self.all_process.get_mut(&child_pid).expect("Hashmap corrupted").parent = Some(Self::REAPER_PID);
            }
        } else {
            eprintln!("... the reaper is die ... RIP ...");
        }

        let pid = self.curr_process_pid;

        self.remove_curr_running();

        // let signal = self.advance_next_process(0);
        self.advance_next_process(0);
        // Switch to the next process
        unsafe {
            _update_process_end_time(self.time_interval.unwrap());

            let new_kernel_esp = self.load_new_context();
            // After loading the new context:
            // If ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
            // If ring0 process -> block temporary interruptible macro

            // if let Some(HANDLED(_) = signal {
            //     GET_RING(new_kernel_esp) == 3 {
            //         APPLY_PENDING_SIGNAL(new_kernel_esp);
            //     } else {
            //         lock_interruptible();
            //     }
            // }

            _exit_resume(new_kernel_esp, pid, status);
        };
    }

    /// Gets the next available Pid for a new process.
    /// current PID attribution depends on the existence of a pid in the `all_process` HashMap.
    /// This is what POSIX-2018 says about it:
    /// 4.14 Process ID Reuse
    /// A process group ID shall not be reused by the system until the process group lifetime ends.
    ///
    /// A process ID shall not be reused by the system until the process lifetime ends. In addition,
    /// if there exists a process group whose process group ID is equal to that process ID, the process
    /// ID shall not be reused by the system until the process group lifetime ends. A process that is not
    /// a system process shall not have a process ID of 1.
    fn get_available_pid(&self) -> Pid {
        fn posix_constraits(_pid: Pid) -> bool {
            true // TODO: We don't have process groups yet so we can't implement the posix requirements
        }

        let pred = |pid| pid > 0 && !self.all_process.contains_key(&pid) && posix_constraits(pid);
        let mut pid = self.next_pid.fetch_add(1, Ordering::Relaxed);

        while !pred(pid) {
            pid = self.next_pid.fetch_add(1, Ordering::Relaxed);
        }
        pid
    }
}

/// Start the whole scheduler
pub unsafe fn start(task_mode: TaskMode) -> ! {
    // Inhibit all hardware interrupts, particulary timer.
    interrupts::disable();

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

    preemptible();
    // After futur IRET for final process creation, interrupt must be re-enabled
    p.start()
}

lazy_static! {
    pub static ref SCHEDULER: Spinlock<Scheduler> = Spinlock::new(Scheduler::new());
}
