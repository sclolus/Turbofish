//! this file contains the scheduler description

use super::process::{get_ring, CpuState, KernelProcess, Process, UserProcess};
use super::signal_interface::JobAction;
use super::syscall::clone::CloneFlags;
use super::task::{ProcessState, Task, WaitingState};
use super::thread_group::ThreadGroup;
use super::{SysResult, TaskMode};
use crate::terminal::ansi_escape_code::Colored;
use alloc::boxed::Box;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::mem;
use core::sync::atomic::{AtomicI32, Ordering};
use errno::Errno;
use fallible_collections::btree::BTreeMap;
use libc_binding::Signum;
use messaging::{MessageTo, ProcessMessage, SchedulerMessage};
use sync::Spinlock;
use terminal::TERMINAL;

use crate::drivers::PIT0;
use crate::interrupts;
use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use crate::system::PrivilegeLevel;

extern "C" {
    fn _exit_resume(new_kernel_esp: u32, process_to_free_pid: Pid, status: i32) -> !;

    fn _auto_preempt() -> i32;

    pub fn _get_pit_time() -> u32;
    pub fn _get_process_end_time() -> u32;

    fn _update_process_end_time(update: u32);

    pub fn _unpreemptible();
    pub fn _preemptible();
    pub fn _schedule_force_preempt();
}

pub type Tid = u32;
pub use libc_binding::Pid;

/// Protect process again scheduler interruption
#[inline(always)]
pub fn unpreemptible() {
    unsafe {
        _unpreemptible();
    }
}

/// Allow scheduler to interrupt process execution
#[inline(always)]
pub fn preemptible() {
    unsafe {
        // Check if the Time to live of the current process is expired
        // TODO: If scheduler is disable, the kernel will crash
        // TODO: After Exit, the next process seems to be skiped !
        if _get_pit_time() >= _get_process_end_time() {
            auto_preempt();
        } else {
            _preemptible();
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

/// Auto-preempt will cause schedule into the next process
/// In some critical cases like signal, avoid this switch
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

    // Store the current kernel stack pointer
    scheduler.store_kernel_esp(kernel_esp);
    scheduler.load_next_process(1)
}

/// Remove ressources of the exited process and note his exit status
#[no_mangle]
unsafe extern "C" fn scheduler_exit_resume(process_to_free_pid: Pid, status: i32) {
    SCHEDULER.force_unlock();

    let mut scheduler = SCHEDULER.lock();

    let dead_process = scheduler
        .get_thread_group_mut(process_to_free_pid)
        .expect("WTF");
    // Send a sig child signal to the father
    if let Some(parent_pid) = dead_process.parent {
        let parent = scheduler.get_task_mut((parent_pid, 0)).expect("WTF");
        //TODO: Announce memory error later.
        mem::forget(parent.signal.generate_signal(Signum::SIGCHLD));
        messaging::push_message(MessageTo::Process {
            pid: parent_pid,
            content: ProcessMessage::ProcessDied {
                pid: process_to_free_pid,
            },
        });
    }
    let dead_process = scheduler.get_thread_group_mut(process_to_free_pid).unwrap();
    dead_process.set_zombie(status);
    preemptible();
}

#[derive(Debug)]
/// Scheduler structure
pub struct Scheduler {
    /// contains a hashmap of pid, process
    all_process: BTreeMap<Pid, ThreadGroup>,
    /// contains pids of all runing process
    running_process: Vec<(Pid, Tid)>,

    /// The next pid to be considered by the scheduler
    /// TODO: think about PID Reuse when SMP will be added,
    /// as current PID attribution depends on the existence of a pid in the
    /// `all_process` HashMap.
    next_pid: AtomicI32,

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
            all_process: BTreeMap::new(),
            next_pid: AtomicI32::new(1),
            current_task_index: 0,
            current_task_id: (1, 0),
            time_interval: None,
            kernel_idle_process: None,
            idle_mode: false,
        }
    }

    /// load the next process, returning the new_kernel_esp
    unsafe fn load_next_process(&mut self, next_process: usize) -> u32 {
        _update_process_end_time(self.time_interval.unwrap());

        self.dispatch_messages();
        // Switch between processes
        let action = self.advance_next_process(next_process);

        // Set all the context of the illigible process
        let new_kernel_esp = self.load_new_context(action);

        // Restore kernel_esp for the new process/
        new_kernel_esp
    }

    fn dispatch_messages(&mut self) {
        // get the keypress from the keybuffer

        for message in messaging::drain_messages() {
            // eprintln!("{:#?}", message);
            match message {
                MessageTo::Process { pid, content } => {
                    self.get_task_mut((pid, 0))
                        .map(|task| task.message_queue.push_back(content));
                }
                MessageTo::Scheduler { content } => match content {
                    SchedulerMessage::SomethingToRead => {
                        if let Some(task) = self
                            .iter_task_mut()
                            .find(|t| t.get_waiting_state() == Some(&WaitingState::Read))
                        {
                            task.message_queue
                                .push_back(ProcessMessage::SomethingToRead);
                        }
                    }
                },
                MessageTo::ProcessGroup {
                    pgid,
                    content: signum,
                } => {
                    for task in self
                        .iter_thread_groups_mut()
                        .filter(|thread_group| thread_group.pgid == pgid)
                        .filter_map(|thread_group| thread_group.get_first_thread())
                    {
                        //TODO: Announce memory error later.
                        mem::forget(task.signal.generate_signal(signum));
                    }
                }
                MessageTo::Tty { key_pressed } => unsafe {
                    TERMINAL
                        .as_mut()
                        .unwrap()
                        .handle_key_pressed(key_pressed, 1);
                },
            }
        }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_user_process(
        &mut self,
        father_pid: Option<Pid>,
        process: Box<UserProcess>,
    ) -> Result<Pid, CollectionAllocErr> {
        let pid = self.get_available_pid();
        self.running_process.try_reserve(1)?;
        self.all_process.try_insert(
            pid,
            ThreadGroup::try_new(
                father_pid,
                Task::new(ProcessState::Running(process)),
                father_pid.unwrap_or(pid),
            )?,
        )?;
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

    /// Advance until a next elligible process was found, modify
    /// self.current_task_index and self.current_task_id
    fn advance_next_process(&mut self, offset: usize) -> JobAction {
        let next_process_index = (self.current_task_index + offset) % self.running_process.len();
        // dbg!(&self.running_process);

        for idx in 0..self.running_process.len() {
            self.current_task_index = (next_process_index + idx) % self.running_process.len();
            self.current_task_id = self.running_process[self.current_task_index];

            // Check if pending signal: Signal Can interrupt all except zombie
            // some signals may be marked as IGNORED, Remove signal and dont DO anything in this case
            // else create a signal var with option<SignalStatus>

            // whether the parent must be wake up
            let p = self.current_task();
            let action = p.signal.get_job_action();

            // Job control: STOP lock thread, CONTINUE (witch erase STOP) or TERMINATE unlock it
            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                continue;
            }
            while let Some(message) = self.current_task_mut().message_queue.pop_front() {
                // eprintln!("Process message: {:#?}", message);
                match message {
                    ProcessMessage::ProcessDied {
                        pid: dead_process_pid,
                    } => {
                        if let Some(WaitingState::ChildDeath(wake_pid, _)) =
                            self.current_task().get_waiting_state()
                        {
                            let dead_process_pgid = self
                                .get_thread_group(dead_process_pid)
                                .expect("no dead child")
                                .pgid;
                            if *wake_pid == -1
                                || *wake_pid == 0
                                    && dead_process_pgid == self.current_thread_group().pgid
                                || *wake_pid == dead_process_pid
                                || -*wake_pid == dead_process_pgid
                            {
                                let status = self
                                        .get_thread_group(dead_process_pid)
                                        .and_then(|tg| tg.get_death_status())
                                        .expect("A zombie was found just before, but there is no zombie here");
                                let current_task = self.current_task_mut();
                                current_task.set_waiting(WaitingState::ChildDeath(
                                    dead_process_pid,
                                    status as u32,
                                ));
                                current_task.set_return_value(0);
                                return JobAction::default();
                            }
                        }
                    }
                    ProcessMessage::SomethingToRead => {
                        let current_task = self.current_task_mut();
                        current_task
                            .message_queue
                            .retain(|message| *message != ProcessMessage::SomethingToRead);
                        current_task.set_return_value(0);
                        current_task.set_running();
                        return JobAction::default();
                    }
                }
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
                        WaitingState::Sleeping(time) => {
                            let now = unsafe { _get_pit_time() };
                            if now >= *time {
                                self.current_task_mut().set_running();
                                self.current_task_mut().set_return_value(0);
                                return action;
                            }
                        }
                        _ => {}
                    }
                }
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

    pub fn current_thread_group(&self) -> &ThreadGroup {
        self.get_thread_group(self.current_task_id.0).unwrap()
    }

    pub fn current_thread_group_mut(&mut self) -> &mut ThreadGroup {
        self.get_thread_group_mut(self.current_task_id.0).unwrap()
    }

    pub fn get_thread_group(&self, pid: Pid) -> Option<&ThreadGroup> {
        self.all_process.get(&pid)
    }

    pub fn get_thread_group_mut(&mut self, pid: Pid) -> Option<&mut ThreadGroup> {
        self.all_process.get_mut(&pid)
    }

    pub fn get_task(&self, id: (Pid, Tid)) -> Option<&Task> {
        self.get_thread_group(id.0)?.get_all_thread()?.get(&id.1)
    }

    pub fn get_task_mut(&mut self, id: (Pid, Tid)) -> Option<&mut Task> {
        self.get_thread_group_mut(id.0)?
            .get_all_thread_mut()?
            .get_mut(&id.1)
    }

    #[allow(dead_code)]
    /// iter on all the thread group
    pub fn iter_thread_groups(&self) -> impl Iterator<Item = &ThreadGroup> {
        self.all_process.values()
    }

    #[allow(dead_code)]
    /// iter on all the thread
    pub fn iter_task(&self) -> impl Iterator<Item = &Task> {
        self.iter_thread_groups()
            .flat_map(|thread_group| thread_group.get_all_thread())
            .flat_map(|all_thread| all_thread.values())
    }

    /// iter on all the thread group mutably
    pub fn iter_thread_groups_mut(&mut self) -> impl Iterator<Item = &mut ThreadGroup> {
        self.all_process.values_mut()
    }

    /// iter on all the thread mutably
    pub fn iter_task_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.iter_thread_groups_mut()
            .flat_map(|thread_group| thread_group.get_all_thread_mut())
            .flat_map(|all_thread| all_thread.values_mut())
    }

    #[allow(dead_code)]
    /// Remove the current running process
    fn remove_curr_running(&mut self) {
        // Remove process from the running process list
        self.running_process.remove(self.current_task_index);
        // Check if there is altmost one process
        //TODO: I think we should panic
        if self.running_process.len() == 0 {
            log::warn!("No more process");
            loop {}
        }
    }

    /// remove all thread belonging to thread group `pid` in the
    /// running list
    fn remove_thread_group_running(&mut self, pid: Pid) {
        self.running_process
            .retain(|(running_pid, _)| *running_pid != pid);
        //TODO: I think we should panic
        if self.running_process.len() == 0 {
            log::warn!("No more process");
            loop {}
        }
    }

    /// remove the thread group from all_process
    pub fn remove_thread_group(&mut self, pid: Pid) {
        self.all_process
            .remove(&pid)
            .expect("remove_thread_goup, thread group doen't exist");
    }

    pub fn current_task_clone(
        &mut self,
        kernel_esp: u32,
        child_stack: *const c_void,
        flags: CloneFlags,
    ) -> SysResult<Pid> {
        if self.time_interval == None {
            panic!("It'a illogical to fork a process when we are in monotask mode");
        }
        self.running_process.try_reserve(1)?;
        let (father_pid, father_tid) = self.current_task_id;

        let child_pid = if flags.contains(CloneFlags::THREAD) {
            let current_task = self.current_task_mut();

            let child = current_task.sys_clone(kernel_esp, child_stack, flags)?;
            let thread_group = self.current_thread_group_mut();
            let tid = thread_group.get_available_tid();
            thread_group
                .get_all_thread_mut()
                .expect("wtf")
                .try_insert(tid, child)?;
            self.running_process.push((father_pid, tid));
            father_pid
        } else {
            let child_pid = self.get_available_pid();
            let thread_group = self.current_thread_group_mut();

            let new_thread_group = thread_group.sys_clone(
                father_pid,
                father_tid,
                child_pid,
                kernel_esp,
                child_stack,
                flags,
            )?;

            self.all_process.try_insert(child_pid, new_thread_group)?;
            self.running_process.push((child_pid, 0));
            child_pid
        };

        Ok(child_pid)
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
            139 => println!("{}", "segmentation fault".red()),
            137 => println!("killed"),
            _ => {}
        }

        // When the father die, the process Self::REAPER_PID adopts all his orphelans
        // TODO: Why we don't add the dead process to the child list of reaper
        if let Some(_reaper) = self.get_task((Self::REAPER_PID, 0)) {
            while let Some(child_pid) = self.current_thread_group_mut().child.pop() {
                self.get_thread_group_mut(child_pid)
                    .expect("Hashmap corrupted")
                    .parent = Some(Self::REAPER_PID);
            }
        } else {
            log::warn!("... the reaper is die ... RIP ...");
        }

        let (pid, _) = self.current_task_id;

        self.remove_thread_group_running(pid);

        // Switch to the next process
        unsafe {
            let new_kernel_esp = self.load_next_process(0);

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
