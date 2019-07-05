//! this file contains the scheduler description

use super::process::{get_ring, CpuState, KernelProcess, Process, UserProcess};
use super::signal::{JobAction, Signum};
use super::syscall::{clone::CloneFlags, read::handle_tty_control};
use super::task::{ProcessState, Task, WaitingState};
use super::{SysResult, TaskMode};
use alloc::collections::CollectionAllocErr;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};
use errno::Errno;
use hashmap_core::fnv::FnvHashMap as HashMap;
use sync::Spinlock;

use crate::drivers::PIT0;
use crate::interrupts;
use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use crate::system::PrivilegeLevel;

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
pub type Tid = u32;

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
        if SIGNAL_LOCK == false {
            // Check if the Time to live of the current process is expired
            // TODO: If scheduler is disable, the kernel will crash
            // TODO: After Exit, the next process seems to be skiped !
            if crate::taskmaster::scheduler::_get_pit_time()
                >= crate::taskmaster::scheduler::_get_process_end_time()
            {
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

/// For signal from RING0, special lock interruptible state while signal(s) is/are not applied
pub static mut SIGNAL_LOCK: bool = false;

/// Auto-preempt will cause schedule into the next process
/// In some critical cases like signal, avoid this switch
pub fn auto_preempt() -> i32 {
    unsafe {
        SCHEDULER.force_unlock();
        if SIGNAL_LOCK == true {
            -1
        } else {
            _auto_preempt()
        }
    }
}

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(kernel_esp: u32) -> u32 {
    let mut scheduler = SCHEDULER.lock();
    _update_process_end_time(scheduler.time_interval.unwrap());

    // Store the current kernel stack pointer
    scheduler.store_kernel_esp(kernel_esp);

    // Handle a tty control
    handle_tty_control();

    // Switch between processes
    let action = scheduler.advance_next_process(1);

    // Set all the context of the illigible process
    let new_kernel_esp = scheduler.load_new_context(action);

    // Restore kernel_esp for the new process/
    new_kernel_esp
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free: Pid, status: i32) {
    SCHEDULER.force_unlock();

    SCHEDULER
        .lock()
        .get_task_mut((process_to_free, 0))
        .unwrap()
        .process_state = ProcessState::Zombie(status);

    preemptible();
}

fn new_thread_list(task: Task) -> Result<HashMap<Tid, Task>, CollectionAllocErr> {
    let mut all_thread = HashMap::new();
    all_thread.try_reserve(1)?;
    all_thread.insert(0, task);
    Ok(all_thread)
}

#[derive(Debug)]
/// Scheduler structure
pub struct Scheduler {
    /// contains a hashmap of pid, process
    pub all_process: HashMap<Pid, HashMap<Tid, Task>>,
    /// contains pids of all runing process
    running_process: Vec<(Pid, Tid)>,

    /// The next pid to be considered by the scheduler
    /// TODO: think about PID Reuse when SMP will be added,
    /// as current PID attribution depends on the existence of a pid in the
    /// `all_process` HashMap.
    next_pid: AtomicU32,

    /// index in the vector of the current running process
    current_task_id: (Pid, Tid),
    /// current process index in the running_process vector
    current_task_index: usize,
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
            current_task_index: 0,
            current_task_id: (1, 0),
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
        self.all_process.insert(
            pid,
            new_thread_list(Task::new(father_pid, ProcessState::Running(process)))?,
        );
        self.running_process.push((pid, 0));
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
                self.kernel_idle_process
                    .as_mut()
                    .expect("No idle mode process")
                    .kernel_esp = kernel_esp;
                self.idle_mode = false;
            }
            false => {
                self.current_task_mut().unwrap_process_mut().kernel_esp = kernel_esp;
            }
        }
    }

    /// Advance until a next elligible process was found
    fn advance_next_process(&mut self, offset: usize) -> JobAction {
        let next_process_index = (self.current_task_index + offset) % self.running_process.len();

        for idx in 0..self.running_process.len() {
            self.current_task_index = (next_process_index + idx) % self.running_process.len();
            self.current_task_id = self.running_process[self.current_task_index];

            // Check if pending signal: Signal Can interrupt all except zombie
            // some signals may be marked as IGNORED, Remove signal and dont DO anything in this case
            // else create a signal var with option<SignalStatus>
            let p = self.current_task_mut();

            let action = p.signal.get_job_action();

            // Job control: STOP lock thread, CONTINUE (witch erase STOP) or TERMINATE unlock it
            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                continue;
            }

            match &self.current_task().process_state {
                ProcessState::Running(_) => return action,
                ProcessState::Waiting(_, waiting_state) => {
                    if action.intersects(JobAction::TERMINATE) {
                        // Immediately resume blocking syscall if TERMINATE action
                        return action;
                    } else if action.intersects(JobAction::INTERRUPT) {
                        // Check if signal var contains something, set return value as
                        // negative (rel to SIGNUM), set process as running then return
                        self.current_task_mut().set_running();
                        self.current_task_mut()
                            .set_return_value(-(Errno::Eintr as i32));
                        return action;
                    }
                    match waiting_state {
                        WaitingState::Event(f) => {
                            if let Some(res) = f() {
                                self.current_task_mut().set_running();
                                // TODO: This is a dummy implementation. If bit 31 of result is set it can lead to undefined behavior
                                self.current_task_mut().set_return_value(res as i32);
                                return action;
                            }
                        }
                        WaitingState::Sleeping(time) => {
                            let now = unsafe { _get_pit_time() };
                            if now >= *time {
                                self.current_task_mut().set_running();
                                self.current_task_mut().set_return_value(0);
                                return action;
                            }
                        }
                        WaitingState::ChildDeath(pid_opt, _) => {
                            let zombie_pid = match pid_opt {
                                // In case of PID == None, Check is the at least one child is a zombie.
                                None => {
                                    if let Some(&zombie_pid) =
                                        self.current_task().child.iter().find(|&current_pid| {
                                            self.get_task((*current_pid, 0))
                                                .expect("Hashmap corrupted")
                                                .is_zombie()
                                        })
                                    {
                                        Some(zombie_pid)
                                    } else {
                                        None
                                    }
                                }
                                // In case of PID >= 0, Check is specified child PID is a zombie.
                                Some(pid) => {
                                    if let Some(elem) = self
                                        .current_task()
                                        .child
                                        .iter()
                                        .find(|&&current_pid| current_pid == *pid as u32)
                                    {
                                        if self
                                            .get_task((*elem, 0))
                                            .expect("Hashmap corrupted")
                                            .is_zombie()
                                        {
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
                                let child = self.get_task((pid, 0)).expect("Hashmap corrupted");
                                match child.process_state {
                                    ProcessState::Zombie(status) => {
                                        self.current_task_mut()
                                            .set_waiting(WaitingState::ChildDeath(zombie_pid, status as u32));
                                        self.current_task_mut().set_return_value(0);
                                        return action;
                                    }
                                    _ => panic!("A zombie was found just before, but there is no zombie here"),
                                };
                            }
                        }
                        _ => {}
                    }
                }
                ProcessState::Zombie(_) => panic!("A zombie cannot be in the running list"),
            };
        }
        self.idle_mode = true;
        JobAction::default()
    }

    /// Prepare the context for the new illigible process
    fn load_new_context(&mut self, action: JobAction) -> u32 {
        match self.idle_mode {
            true => {
                let process = self.kernel_idle_process.as_ref();
                process.expect("No idle mode process").kernel_esp
            }
            false => {
                let p = self.current_task_mut();

                let process = p.unwrap_process();
                unsafe {
                    process.context_switch();
                }
                let kernel_esp = process.kernel_esp;

                // If ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
                // If ring0 process -> block temporary interruptible macro
                let ring = unsafe { get_ring(kernel_esp) };
                if action.intersects(JobAction::TERMINATE)
                    || action.intersects(JobAction::INTERRUPT)
                {
                    if ring == PrivilegeLevel::Ring3 {
                        self.current_task_deliver_pending_signals(
                            kernel_esp as *mut CpuState,
                            Scheduler::NOT_IN_BLOCKED_SYSCALL,
                        )
                    } else {
                        unsafe {
                            SIGNAL_LOCK = true;
                        }
                    }
                }
                kernel_esp
            }
        }
    }

    /// Get current process pid
    pub fn current_task_id(&self) -> (Pid, Tid) {
        self.current_task_id
    }

    /// Get current process
    pub fn current_task(&self) -> &Task {
        self.get_task(self.current_task_id).unwrap()
    }

    /// Get current process mutably
    pub fn current_task_mut(&mut self) -> &mut Task {
        self.get_task_mut(self.current_task_id).unwrap()
    }

    pub fn get_task(&self, id: (Pid, Tid)) -> Option<&Task> {
        self.all_process.get(&id.0)?.get(&id.1)
    }

    pub fn get_task_mut(&mut self, id: (Pid, Tid)) -> Option<&mut Task> {
        self.all_process.get_mut(&id.0)?.get_mut(&id.1)
    }

    /// Remove the current running process
    fn remove_curr_running(&mut self) {
        // Remove process from the running process list
        self.running_process.remove(self.current_task_index);
        // Check if there is altmost one process
        if self.running_process.len() == 0 {
            log::warn!("No more process");
            loop {}
        }
    }

    /// Perform a fork
    pub fn current_task_fork(&mut self, kernel_esp: u32) -> SysResult<u32> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        self.all_process.try_reserve(1)?;
        self.running_process.try_reserve(1)?;
        let child_pid = self.get_available_pid();
        let father_pid = self.current_task_id.0;
        let current_task = self.current_task_mut();
        current_task.child.try_reserve(1)?;

        // try reserve a place for child pid

        let child = current_task.fork(kernel_esp, father_pid)?;

        self.all_process.insert(child_pid, new_thread_list(child)?);
        self.running_process.push((child_pid, 0));

        self.current_task_mut().child.push(child_pid);
        // dbg!(self.current_task());

        Ok(child_pid)
    }

    pub fn current_task_clone(
        &mut self,
        kernel_esp: u32,
        _function: u32,
        _child_stack: *const c_void,
        flags: CloneFlags,
        _args: *const c_void,
    ) -> SysResult<u32> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        // self.all_process.try_reserve(1)?;
        // self.running_process.try_reserve(1)?;
        // let child_pid = self.get_available_pid();
        let father_pid = self.current_task_id.0;
        let current_task = self.current_task_mut();
        // current_task.child.try_reserve(1)?;

        // // try reserve a place for child pid

        let _child = current_task.sys_clone(kernel_esp, father_pid, flags)?;

        // self.all_process.insert(child_pid, new_thread_list(child)?);
        // self.running_process.push((child_pid, 0));

        // self.current_task_mut().child.push(child_pid);
        // dbg!(self.current_task());

        // Ok(child_pid)
        unimplemented!()
    }

    const REAPER_PID: Pid = 1;

    /// Exit form a process and go to the current process
    pub fn current_task_exit(&mut self, status: i32) -> ! {
        log::info!(
            "exit called for process with PID: {:?} STATUS: {:?}",
            self.running_process[self.current_task_index],
            status
        );

        match status {
            139 => println!("segmentation fault"),
            137 => println!("killed"),
            _ => {}
        }

        // When the father die, the process 0 adopts all his orphelans
        if let Some(reaper) = self.get_task((Self::REAPER_PID, 0)) {
            if let ProcessState::Zombie(_) = reaper.process_state {
                log::warn!("... the reaper is a zombie ... it is worring ...");
            }
            while let Some(child_pid) = self.current_task_mut().child.pop() {
                self.get_task_mut((child_pid, 0))
                    .expect("Hashmap corrupted")
                    .parent = Some(Self::REAPER_PID);
            }
        } else {
            log::warn!("... the reaper is die ... RIP ...");
        }

        let pid = self.current_task_id.0;

        // Send a sig child signal to the father
        if let Some(parent_pid) = self.current_task().parent {
            let parent = self.get_task_mut((parent_pid, 0)).expect("WTF");
            let _ret = parent.signal.generate_signal(Signum::Sigchld);
        }

        self.remove_curr_running();

        let signal = self.advance_next_process(0);

        // Switch to the next process
        unsafe {
            _update_process_end_time(self.time_interval.unwrap());

            let new_kernel_esp = self.load_new_context(signal);

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

    /// Usable for external caller to announce we not go from a blocked syscall
    pub const NOT_IN_BLOCKED_SYSCALL: bool = false;

    /// apply pending signal, must be called when process is in ring 3
    pub fn current_task_deliver_pending_signals(
        &mut self,
        cpu_state: *mut CpuState,
        in_blocked_syscall: bool,
    ) {
        debug_assert_eq!(
            unsafe { get_ring(cpu_state as u32) },
            PrivilegeLevel::Ring3,
            "Cannot apply signal from ring0 process"
        );
        let signum: Option<Signum> = self
            .current_task_mut()
            .signal
            .exec_signal_handler(cpu_state, in_blocked_syscall);
        if let Some(signum) = signum {
            self.current_task_exit(signum as i32 + 128);
        }
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
            log::info!(
                "Scheduler initialised at frequency: {:?} hz",
                scheduler_frequency
            );
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
    let p = scheduler.current_task_mut().unwrap_process_mut();

    // force unlock the scheduler as process borrows it and we won't get out of scope
    SCHEDULER.force_unlock();

    log::info!("Starting processes");

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
