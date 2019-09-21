//! this file contains the scheduler description
use super::process::{get_ring, CpuState, KernelProcess, Process, ProcessOrigin, UserProcess};
use super::signal_interface::JobAction;
use super::sync::SmartMutex;
use super::syscall::clone::CloneFlags;
use super::syscall::kernel_mod::KernelModules;
use super::thread::{AutoPreemptReturnValue, ProcessState, Thread, WaitingState};
use super::thread_group::{RunningThreadGroup, Status, ThreadGroup};
use super::{SysResult, TaskMode};

mod dustman;
use dustman::{dustman_handler, DUSTMAN_TRIGGER};

use alloc::boxed::Box;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::{AtomicI32, Ordering};
use fallible_collections::btree::BTreeMap;
use fallible_collections::FallibleVec;
use i386::PrivilegeLevel;
use interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use libc_binding::Errno;
use libc_binding::Signum;
use messaging::{MessageTo, ProcessGroupMessage, ProcessMessage};
use terminal::TERMINAL;

use crate::drivers::PIT0;
use crate::terminal::ansi_escape_code::Colored;

/// These extern functions are coded in low level assembly. They are 'arch specific i686'
extern "C" {
    /// Usable by blocking syscalls. 'Freeze' a given proces then switch to another process.
    fn _auto_preempt() -> i32;

    /// This function function is associated to _auto_preempt() and will be handled by IRQ 81
    fn _schedule_force_preempt();

    /// Get the pit realtime.
    fn _get_pit_time() -> u32;

    /// Get the process avalable time to life.
    fn _get_process_end_time() -> u32;

    /// Give time to life for the next launched process.
    fn _update_process_end_time(update: u32);

    /// Prevent the current execution thread by some scheduler interrupt.
    /// Return the current preemptif status
    /// 0 => UNLOCKED
    /// 1 => LOCKED
    fn _unpreemptible() -> u32;

    /// Allow the current execution thread to be interruptible by the scheduler again.
    fn _preemptible();

    /// Idle process
    static _idle_process_len: usize;
    fn _idle_process_code();
}

pub type Tid = u32;
pub use libc_binding::Pid;

use core::fmt::{self, Debug};

impl Debug for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scheduler")
    }
}

/// Scheduler structure
pub struct Scheduler {
    /// contains status of kernel modules
    pub kernel_modules: KernelModules,
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
    kernel_idle_process: Box<KernelProcess>,
    /// DustMan: The process Ripper
    dustman: Box<KernelProcess>,
    /// Current mode of the scheduler
    mode: Mode,
    /// Indicate if scheduler is on exit routine
    pub on_exit_routine: Option<(Pid, Status)>,
}

/// The pit handler (cpu_state represents a pointer to esp)
#[no_mangle]
unsafe extern "C" fn scheduler_interrupt_handler(kernel_esp: u32) -> u32 {
    let mut scheduler = SCHEDULER.lock();

    if let Some(_) = scheduler.on_exit_routine {
        scheduler.on_exit_routine = None;
    }

    // Store the current kernel stack pointer
    scheduler.store_kernel_esp(kernel_esp);

    // Switch to the next elligible process then return new kernel ESP
    scheduler.load_next_process(1)
}

#[derive(Debug, Copy, Clone)]
enum Mode {
    Normal,
    Idle,
    DustMan,
}

/// Base Scheduler implementation
impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            kernel_modules: KernelModules::new(),
            running_process: Vec::new(),
            all_process: BTreeMap::new(),
            next_pid: AtomicI32::new(1),
            current_task_index: 0,
            current_task_id: (1, 0),
            time_interval: None,
            kernel_idle_process: unsafe {
                KernelProcess::new(
                    ProcessOrigin::Raw(_idle_process_code as *const u8, _idle_process_len),
                    None,
                )
                .expect("Cannot assign Idle process to scheduler")
            },
            dustman: unsafe {
                KernelProcess::new(ProcessOrigin::KernelFunction(dustman_handler), None)
                    .expect("Cannot assign Idle process to scheduler")
            },
            mode: Mode::Normal,
            on_exit_routine: None,
        }
    }

    /// load the next process, returning the new_kernel_esp
    unsafe fn load_next_process(&mut self, next_process: usize) -> u32 {
        _update_process_end_time(self.time_interval.unwrap());

        self.dispatch_messages();
        // Switch between processes
        let action = self.advance_next_process(next_process);
        // Set all the context of the illigible process
        self.load_new_context(action)
    }

    fn dispatch_messages(&mut self) {
        // get the keypress from the keybuffer

        for message in messaging::drain_messages() {
            // eprintln!("{:#?}", message);
            match message {
                MessageTo::Tty { key_pressed } => unsafe {
                    let _r = TERMINAL.as_mut().unwrap().handle_key_pressed(key_pressed);
                },
                _ => panic!("message not covered"),
            }
        }
    }

    /// Add a process into the scheduler (transfert ownership)
    pub fn add_user_process(
        &mut self,
        father_pid: Pid,
        process: Box<UserProcess>,
    ) -> Result<Pid, CollectionAllocErr> {
        let pid = self.get_available_pid();
        self.running_process.try_reserve(1)?;
        self.all_process.try_insert(
            pid,
            ThreadGroup::try_new(
                father_pid,
                Thread::new(ProcessState::Running(process))?,
                pid,
            )?,
        )?;
        self.running_process.push((pid, 0));
        Ok(pid)
    }

    /// Backup of the current process kernel_esp
    fn store_kernel_esp(&mut self, kernel_esp: u32) {
        use Mode::*;
        match self.mode {
            Normal => {
                self.current_thread_mut().unwrap_process_mut().kernel_esp = kernel_esp;
            }
            Idle => {
                self.kernel_idle_process.kernel_esp = kernel_esp;
            }
            DustMan => {
                self.dustman.kernel_esp = kernel_esp;
            }
        }
        self.mode = Normal;
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
            let action = self.current_thread_get_job_action();

            // Job control: STOP lock thread, CONTINUE (witch erase STOP) or TERMINATE unlock it
            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                continue;
            }
            match &self.current_thread().process_state {
                ProcessState::Running(_) => return action,
                ProcessState::Waiting(_, waiting_state) => {
                    if action.intersects(JobAction::TERMINATE) {
                        // Immediately resume blocking syscall if TERMINATE action.
                        // Ce n'est pas tout a fait logique mais on va faire la meme
                        // chose que pour un signal tout con. Le fait de mettre Errno
                        // a EINTR sert juste a retourner une erreur pour debloquer
                        // auto-preempt(). Le deliver_pending_signal machin au fond de syscall
                        // fera le reste du boulot pour achever le process.
                        // Il y a toujours un truc qui me chaffouine profondement la-dedans...
                        self.current_thread_mut().set_running();
                        self.current_thread_mut()
                            .set_return_value_autopreempt(Err(Errno::EINTR));
                        return action;
                    } else if action.intersects(JobAction::INTERRUPT) {
                        // Check if signal var contains something, set return value as
                        // Err(Errno::EINTR), set process as running then return
                        self.current_thread_mut().set_running();
                        self.current_thread_mut()
                            .set_return_value_autopreempt(Err(Errno::EINTR));
                        return action;
                    }
                    match waiting_state {
                        WaitingState::Sleeping(time) => {
                            let now = unsafe { _get_pit_time() };
                            if now >= *time {
                                self.current_thread_mut().set_running();
                                self.current_thread_mut()
                                    .set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                                return action;
                            }
                        }
                        _ => {}
                    }
                }
            };
        }
        self.mode = Mode::Idle;
        JobAction::default()
    }

    /// Prepare the context for the new illigible process
    fn load_new_context(&mut self, action: JobAction) -> u32 {
        use Mode::*;
        match self.mode {
            DustMan => self.dustman.kernel_esp,
            Idle => self.kernel_idle_process.kernel_esp,
            Normal => {
                let p = self.current_thread_mut();

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
                        if let Some(_) = self.current_thread_deliver_pending_signals(
                            kernel_esp as *mut CpuState,
                            Scheduler::NOT_IN_BLOCKED_SYSCALL,
                        ) {
                            // An exit() routine may be engaged after handling a deadly signal
                            return self.set_dustman_mode();
                        }
                    }
                }
                kernel_esp
            }
        }
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

    pub fn current_thread_clone(
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
            let current_thread = self.current_thread_mut();

            let child = current_thread.sys_clone(kernel_esp, child_stack, flags)?;
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

    /// Start the exit() routine: Return informations about the process to destroy
    pub fn current_thread_group_exit(&mut self, status: Status) -> Option<(Pid, Status)> {
        log::info!(
            "exit called for process with PID: {:?} STATUS: {:?}",
            self.running_process[self.current_task_index],
            status
        );

        match status {
            Status::Signaled(Signum::SIGSEGV) => println!("{}", "segmentation fault".red()),
            Status::Signaled(signum) => println!("killed by signal: {:?}", signum),
            _ => {}
        }

        while let Some(child_pid) = self.current_thread_group_running_mut().child.pop() {
            use super::thread_group::ThreadGroupState;

            let thread_group = self
                .get_thread_group_mut(child_pid)
                .expect("Hashmap corrupted");
            match thread_group.thread_group_state {
                // If the child is on RunningState. The REAPER must adopt him
                ThreadGroupState::Running(_) => {
                    thread_group.parent = Self::REAPER_PID;

                    let _r = self
                        .get_thread_group_mut(Self::REAPER_PID)
                        .expect("no reaper process")
                        .unwrap_running_mut()
                        .child
                        .try_push(child_pid)
                        .map_err(|_| {
                            log::error!("no memory to push on the reaper pid: Trashing zombie");
                            self.remove_thread_group(child_pid);
                        });
                }
                // Else, definitively destroy it
                ThreadGroupState::Zombie(_) => {
                    self.remove_thread_group(child_pid);
                }
            }
        }

        let (pid, _) = self.current_task_id;

        self.remove_thread_group_running(pid);

        DUSTMAN_TRIGGER.store(true, Ordering::Relaxed);
        unpreemptible();

        self.on_exit_routine = Some((pid, status));
        self.on_exit_routine
    }

    /// Finalize the exit() routine: Remove ressources of the exited process and send his exit status
    fn exit_resume(&mut self, process_to_free_pid: Pid, status: Status) {
        // Get a reference to the dead process
        let dead_process = self
            .get_thread_group_mut(process_to_free_pid)
            .expect("WTF: No Dead Process");

        // Send a sig child signal to the father
        let parent_pid = dead_process.parent;
        let parent = self
            .get_thread_mut((parent_pid, 0))
            .expect("WTF: Parent not alive");

        //TODO: Announce memory error later.
        let _ignored_result = parent.signal.generate_signal(Signum::SIGCHLD);

        // Set the dead process as zombie
        let dead_process = self
            .get_thread_group_mut(process_to_free_pid)
            .expect("WTF: No Dead Process");

        let dead_process_pgid = dead_process.pgid;

        // Call the drop chain of file_descriptor_interface before being a zombie !
        dead_process
            .unwrap_running_mut()
            .file_descriptor_interface
            .delete();

        dead_process.set_zombie(status);

        // Send a death testament message to the parent
        self.send_message(MessageTo::Process {
            pid: parent_pid,
            content: ProcessMessage::ProcessUpdated {
                pid: process_to_free_pid,
                pgid: dead_process_pgid,
                status: status.into(),
            },
        });
    }

    /// Call the DustMan to trash a process
    pub fn set_dustman_mode(&mut self) -> u32 {
        self.mode = Mode::DustMan;
        // Don't forget to switch on the DustMan CR3/PD Context
        unsafe {
            self.dustman.context_switch();
        }
        self.dustman.kernel_esp
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
        // this check if the candidate does't pid match any active process group
        let posix_constraits =
            |pid: Pid| -> bool { !self.iter_thread_groups().any(|pg| pg.pgid == pid) };

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
    /// Eventually. Returns informations about a process on killing
    pub fn current_thread_deliver_pending_signals(
        &mut self,
        cpu_state: *mut CpuState,
        in_blocked_syscall: bool,
    ) -> Option<(Pid, Status)> {
        debug_assert_eq!(
            unsafe { get_ring(cpu_state as u32) },
            PrivilegeLevel::Ring3,
            "Cannot apply signal from ring0 process"
        );
        // Get the user_stack location to ensure that are large available space
        let user_stack_range = self.current_thread().unwrap_process().user_stack_range;
        let signum: Option<Signum> = self.current_thread_mut().signal.exec_signal_handler(
            cpu_state,
            user_stack_range,
            in_blocked_syscall,
        );
        if let Some(signum) = signum {
            self.current_thread_group_exit(Status::Signaled(signum))
        } else {
            None
        }
    }

    /// Update the Job process state regarding to the get_job_action() return value
    pub fn current_thread_get_job_action(&mut self) -> JobAction {
        let pid = self.current_task_id.0;
        let current = self.current_thread();
        let action = current.signal.get_job_action();
        let current_thread_group = self.current_thread_group_mut();
        let pgid = current_thread_group.pgid;
        let parent_pid = current_thread_group.parent;
        if action != JobAction::TERMINATE {
            if action == JobAction::STOP {
                if current_thread_group.job.try_set_stoped() {
                    self.send_message(MessageTo::Process {
                        pid: parent_pid,
                        content: ProcessMessage::ProcessUpdated {
                            pid: pid,
                            pgid: pgid,
                            status: Status::Stopped.into(),
                        },
                    });
                }
            } else {
                if current_thread_group.job.try_set_continued() {
                    self.send_message(MessageTo::Process {
                        pid: parent_pid,
                        content: ProcessMessage::ProcessUpdated {
                            pid: pid,
                            pgid: pgid,
                            status: Status::Continued.into(),
                        },
                    });
                }
            }
        }
        action
    }
    /// Get current process pid
    pub fn current_task_id(&self) -> (Pid, Tid) {
        self.current_task_id
    }

    /// Get current process
    pub fn current_thread(&self) -> &Thread {
        self.get_thread(self.current_task_id)
            .expect("wtf current thread doesn't exist")
    }

    /// Get current process mutably
    pub fn current_thread_mut(&mut self) -> &mut Thread {
        self.get_thread_mut(self.current_task_id)
            .expect("wtf current thread doesn't exist")
    }

    pub fn current_thread_group(&self) -> &ThreadGroup {
        self.get_thread_group(self.current_task_id.0)
            .expect("wtf current thread group doesn't exist")
    }

    pub fn current_thread_group_mut(&mut self) -> &mut ThreadGroup {
        self.get_thread_group_mut(self.current_task_id.0)
            .expect("wtf current thread group doesn't exist")
    }

    pub fn get_thread_group(&self, pid: Pid) -> Option<&ThreadGroup> {
        self.all_process.get(&pid)
    }

    pub fn get_thread_group_mut(&mut self, pid: Pid) -> Option<&mut ThreadGroup> {
        self.all_process.get_mut(&pid)
    }

    #[allow(dead_code)]
    pub fn current_thread_group_running(&self) -> &RunningThreadGroup {
        self.current_thread_group().unwrap_running()
    }

    pub fn current_thread_group_running_mut(&mut self) -> &mut RunningThreadGroup {
        self.current_thread_group_mut().unwrap_running_mut()
    }

    pub fn get_thread(&self, id: (Pid, Tid)) -> Option<&Thread> {
        self.get_thread_group(id.0)?.get_all_thread()?.get(&id.1)
    }

    pub fn get_thread_mut(&mut self, id: (Pid, Tid)) -> Option<&mut Thread> {
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
    pub fn iter_thread(&self) -> impl Iterator<Item = &Thread> {
        self.iter_thread_groups()
            .flat_map(|thread_group| thread_group.get_all_thread())
            .flat_map(|all_thread| all_thread.values())
    }

    /// iter on all the thread group mutably
    pub fn iter_thread_groups_mut(&mut self) -> impl Iterator<Item = &mut ThreadGroup> {
        self.all_process.values_mut()
    }

    /// iter on all the thread mutably
    #[allow(dead_code)]
    pub fn iter_thread_mut(&mut self) -> impl Iterator<Item = &mut Thread> {
        self.iter_thread_groups_mut()
            .flat_map(|thread_group| thread_group.iter_thread_mut())
    }

    pub fn send_message(&mut self, message: MessageTo) {
        use super::syscall::WaitOption;
        log::info!("{:?}", message);
        match message {
            MessageTo::Reader { uid_file_op } => {
                self.iter_thread_mut()
                    .find(|thread| {
                        thread.get_waiting_state() == Some(&WaitingState::Read(uid_file_op))
                    })
                    .map(|thread| {
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                        thread.set_running();
                    });
            }
            MessageTo::Accepter { uid_file_op } => {
                self.iter_thread_mut()
                    .find(|thread| {
                        thread.get_waiting_state() == Some(&WaitingState::Accept(uid_file_op))
                    })
                    .map(|thread| {
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                        thread.set_running();
                    });
            }
            MessageTo::Connecter { uid_file_op } => {
                self.iter_thread_mut()
                    .find(|thread| {
                        thread.get_waiting_state() == Some(&WaitingState::Connect(uid_file_op))
                    })
                    .map(|thread| {
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                        thread.set_running();
                    });
            }
            MessageTo::Writer { uid_file_op } => {
                self.iter_thread_mut()
                    .find(|thread| {
                        thread.get_waiting_state() == Some(&WaitingState::Write(uid_file_op))
                    })
                    .map(|thread| {
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                        thread.set_running();
                    });
            }
            MessageTo::Opener { uid_file_op } => {
                self.iter_thread_mut()
                    .find(|thread| {
                        thread.get_waiting_state() == Some(&WaitingState::Open(uid_file_op))
                    })
                    .map(|thread| {
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::None));
                        thread.set_running();
                    });
            }
            MessageTo::Process { pid, content } => match content {
                ProcessMessage::ProcessUpdated {
                    pid: dead_process_pid,
                    pgid: dead_process_pgid,
                    status,
                } => {
                    // to avoid the borrow checker we declare a bool
                    // which comes to true if we find the father Waiting
                    // with the right options
                    let mut finded = false;
                    let s: Status = status.into();
                    if let Some(thread) = self
                        .get_thread_group_mut(pid)
                        .iter_mut()
                        .flat_map(|thread| thread.iter_thread_mut())
                        .find(|thread| {
                            /* Wake Condition of the Waitpid */
                            if let Some(WaitingState::Waitpid {
                                pid: wake_pid,
                                pgid,
                                options,
                            }) = thread.get_waiting_state()
                            {
                                ((options.contains(WaitOption::WUNTRACED) && s == Status::Stopped)
                                    || (options.contains(WaitOption::WCONTINUED)
                                        && s == Status::Continued)
                                    || s.is_exited()
                                    || s.is_signaled())
                                    && (*wake_pid == -1
                                        || *wake_pid == 0 && dead_process_pgid == *pgid
                                        || *wake_pid == dead_process_pid
                                        || -*wake_pid == dead_process_pgid)
                            } else {
                                false
                            }
                            /* end Wake Condition of the Waitpid */
                        })
                    {
                        finded = true;
                        thread.set_running();
                        thread.set_return_value_autopreempt(Ok(AutoPreemptReturnValue::Wait {
                            dead_process_pid,
                            status: s,
                        }));
                    }

                    if finded && (s == Status::Stopped || s == Status::Continued) {
                        // consume the state, because at the return of
                        // auto_preempt after scheduling, the state
                        // can change and it maybe too late to consume
                        // the state
                        self.get_thread_group_mut(dead_process_pid)
                            .expect("no dead pid")
                            .job
                            .consume_last_event()
                            .expect("no status after autopreempt");
                    }
                }
                _ => panic!("message not covered"),
            },
            MessageTo::ProcessGroup { pgid, content } => {
                for thread_group in self.iter_thread_groups_mut().filter(|t| t.pgid == pgid) {
                    match content {
                        ProcessGroupMessage::Signal(signum) => {
                            //TODO: Announce memory error later.

                            thread_group.get_first_thread().map(|thread| {
                                let _ignored_result = thread.signal.generate_signal(signum);
                            });
                        }
                        _ => panic!("message not covered"),
                    }
                }
            }
            _ => panic!("message not covered"),
        }
    }
}

#[no_mangle]
#[allow(dead_code)]
pub unsafe fn get_current_pgid() -> Pid {
    SCHEDULER.force_unlock();
    SCHEDULER.lock().current_thread_group().pgid
}

#[no_mangle]
pub unsafe extern "C" fn send_message(message: MessageTo) {
    SCHEDULER.force_unlock();
    SCHEDULER.lock().send_message(message);
}

/// Start the whole scheduler
pub unsafe fn start(task_mode: TaskMode) -> ! {
    // Inhibit all hardware interrupts, particulary timer.
    interrupts::disable();

    // Register a new IDT entry in 81h for force preempting
    let mut interrupt_table = InterruptTable::current_interrupt_table();

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
        TaskMode::Multi(mut scheduler_frequency) => {
            let mut period = (PIT0.lock().get_frequency().unwrap() / scheduler_frequency) as u32;
            // Be carefull, due to scheduler latency, the minimal period between two Schedule must be 2 tics
            // When we take a long time in a IRQ(x), the next IRQ(x) will be stacked and will be triggered immediatly,
            // That can reduce the time to live of a process to 0 ! (may inhibit auto-preempt mechanism and other things)
            // this is a critical point. Never change that without a serious good reason.
            if period < 2 {
                log::warn!("Too high frequency detected ! Setting period divisor to 2");
                scheduler_frequency = PIT0.lock().get_frequency().unwrap() / 2.;
                period = 2;
            }
            log::info!(
                "Scheduler initialised at frequency: {:?} hz",
                scheduler_frequency
            );
            Some(period)
        }
    };

    let mut scheduler = SCHEDULER.lock();

    scheduler.time_interval = t;

    // Initialise the idle process and get a reference on it
    // Give it a pointer to the function _preemptible()
    scheduler.mode = Mode::DustMan;
    let p = &scheduler.dustman;

    // force unlock the scheduler as process borrows it and we won't get out of scope
    SCHEDULER.force_unlock();

    log::info!("Starting processes");

    match t {
        Some(v) => _update_process_end_time(v),
        None => _update_process_end_time(-1 as i32 as u32),
    }

    // After futur IRET for final process creation, interrupt must be re-enabled
    p.start()
}

lazy_static! {
    pub static ref SCHEDULER: SmartMutex<Scheduler> = SmartMutex::new(Scheduler::new());
}

/// Auto-preempt will cause schedule into the next process
/// In some critical cases like signal, avoid this switch
pub fn auto_preempt() -> SysResult<AutoPreemptReturnValue> {
    unsafe {
        SCHEDULER.force_unlock();
        let ret = _auto_preempt() as *const SysResult<AutoPreemptReturnValue>;
        *ret
    }
}

/// Protect process again scheduler interruption
/// Return the current preemptive status
/// 0 => UNLOCKED
/// 1 => LOCKED
#[inline(always)]
pub fn unpreemptible() -> bool {
    unsafe { _unpreemptible() != 0 }
}

// TODO: If scheduler is disable, the kernel will crash
// TODO: After Exit, the next process seems to be skiped !
/// Allow scheduler to interrupt process execution
#[inline(always)]
pub fn preemptible() {
    unsafe {
        // Check if the Time to live of the current process is expired
        if _get_pit_time() >= _get_process_end_time() {
            // Go to the next elligible process
            let _ignored_result = auto_preempt();
        } else {
            // Just reallow scheduler interrupt
            _preemptible();
        }
    }
}

/// A Finalizer-pattern Struct that disables preemption upon instantiation.
/// then reenables it at Drop time.
pub struct PreemptionGuard {
    /// Contains information about the preemptible state when the new() method is invocated
    already_unpreemptible: bool,
}

impl PreemptionGuard {
    /// The instantiation methods that disables preemption and creates the guard.
    pub fn new() -> Self {
        Self {
            already_unpreemptible: unpreemptible(),
        }
    }

    /// Overwrite the lock as already locked
    pub fn set_already_unpreemptible(&mut self) {
        self.already_unpreemptible = true;
    }
}

impl Drop for PreemptionGuard {
    /// The drop implementation of the guard reenables preemption.
    fn drop(&mut self) {
        if !self.already_unpreemptible {
            preemptible();
        }
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
        let _ret = { $code };
        #[allow(unreachable_code)]
        _ret
    }};
}
