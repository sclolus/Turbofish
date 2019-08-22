//! This file contains definition of a task

use super::process::{CpuState, UserProcess};
use super::scheduler::Pid;
use super::signal_interface::{JobAction, SignalInterface};
use super::syscall::clone::CloneFlags;
use super::SysResult;

use core::ffi::c_void;
use fallible_collections::FallibleBox;

use alloc::boxed::Box;

use core::mem;

#[derive(Debug, Copy, Clone)]
pub enum AutoPreemptReturnValue {
    None,
    Wait { dead_process_pid: Pid, status: i32 },
}

impl Default for AutoPreemptReturnValue {
    fn default() -> Self {
        Self::None
    }
}

/// Main Task definition
#[derive(Debug)]
pub struct Thread {
    /// Current process state
    pub process_state: ProcessState,
    /// Signal Interface
    pub signal: SignalInterface,
    /// Current job status of a process
    pub job: Job,
    /// Return value for auto_preempt
    autopreempt_return_value: Box<SysResult<AutoPreemptReturnValue>>,
}

impl Thread {
    pub fn new(process_state: ProcessState) -> Self {
        Self {
            process_state,
            signal: SignalInterface::new(),
            job: Job::new(),
            autopreempt_return_value: Box::new(Ok(Default::default())),
        }
    }

    pub fn get_waiting_state(&self) -> Option<&WaitingState> {
        match &self.process_state {
            ProcessState::Waiting(_, waiting_state) => Some(waiting_state),
            _ => None,
        }
    }

    pub fn sys_clone(
        &self,
        kernel_esp: u32,
        child_stack: *const c_void,
        flags: CloneFlags,
    ) -> SysResult<Self> {
        Ok(Self {
            signal: self.signal.fork(),
            process_state: match &self.process_state {
                ProcessState::Running(p) => {
                    ProcessState::Running(p.sys_clone(kernel_esp, child_stack, flags)?)
                }
                _ => panic!("Non running process should not clone"),
            },
            job: Job::new(),
            autopreempt_return_value: Box::try_new(Ok(Default::default()))?,
        })
    }

    pub fn unwrap_process_mut(&mut self) -> &mut UserProcess {
        match &mut self.process_state {
            ProcessState::Waiting(process, _) | ProcessState::Running(process) => process,
        }
    }

    pub fn unwrap_process(&self) -> &UserProcess {
        match &self.process_state {
            ProcessState::Running(process) | ProcessState::Waiting(process, _) => process,
        }
    }

    pub fn set_return_value_autopreempt(
        &mut self,
        return_value: SysResult<AutoPreemptReturnValue>,
    ) {
        let cpu_state = self.unwrap_process().kernel_esp as *mut CpuState;
        *self.autopreempt_return_value = return_value;
        unsafe {
            (*(cpu_state)).registers.eax = self.autopreempt_return_value.as_ref()
                as *const SysResult<AutoPreemptReturnValue>
                as u32;
        }
    }

    /// Update the Job process state regarding to the get_job_action() return value
    pub fn get_job_action(&mut self) -> JobAction {
        let action = self.signal.get_job_action();
        if action != JobAction::TERMINATE {
            if action == JobAction::STOP {
                if self.job.try_set_stoped() {
                    // TODO: Send a message to father PID
                }
            } else {
                if self.job.try_set_continued() {
                    // TODO: Send a message to father PID
                }
            }
        }
        action
    }

    // /// For blocking call, set the return value witch will be transmitted by auto_preempt fn
    // pub fn set_return_value(&self, return_value: i32) {
    //     let cpu_state = self.unwrap_process().kernel_esp as *mut CpuState;
    //     unsafe {
    //         (*(cpu_state)).registers.eax = return_value as u32;
    //     }
    // }

    #[allow(dead_code)]
    pub fn is_waiting(&self) -> bool {
        match self.process_state {
            ProcessState::Waiting(_, _) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        match self.process_state {
            ProcessState::Running(_) => true,
            _ => false,
        }
    }

    pub fn set_waiting(&mut self, waiting_state: WaitingState) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_waiting(waiting_state);
        let uninit = mem::replace(&mut self.process_state, next);
        mem::forget(uninit);
    }

    pub fn set_running(&mut self) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_running();
        let uninit = mem::replace(&mut self.process_state, next);
        mem::forget(uninit);
    }
}

#[derive(Debug, PartialEq)]
pub enum WaitingState {
    /// The Process is sleeping until pit time >= u32 value
    Sleeping(u32),
    /// The sys_pause command was invoqued, the process is waiting for a signal
    Pause,
    /// The Process is looking for the death of his child
    Waitpid { pid: Pid, pgid: Pid, options: u32 },
    /// In Waiting to read
    Read,
    /// In Waiting to write
    Write,
    // Event(fn() -> Option<u32>),
}

#[derive(Debug)]
pub enum ProcessState {
    /// The process is currently on running state
    Running(Box<UserProcess>),
    /// The process is currently waiting for something
    Waiting(Box<UserProcess>, WaitingState),
}

impl ProcessState {
    pub fn set_waiting(self, waiting_state: WaitingState) -> Self {
        match self {
            ProcessState::Running(p) => ProcessState::Waiting(p, waiting_state),
            ProcessState::Waiting(p, _) => ProcessState::Waiting(p, waiting_state),
        }
    }
    pub fn set_running(self) -> Self {
        match self {
            ProcessState::Waiting(p, _) => ProcessState::Running(p),
            _ => panic!("already running"),
        }
    }
}

/// State of a process in the point of view of JobAction
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum JobState {
    Stoped,
    Continued,
}

/// Mais Job structure
#[derive(Debug)]
pub struct Job {
    /// Current JobState
    state: JobState,
    /// Last change state (this event may be consumed by waitpid)
    last_event: Option<JobState>,
}

/// Main Job implementation
impl Job {
    const fn new() -> Self {
        Self {
            state: JobState::Continued,
            last_event: None,
        }
    }
    /// Try to set as continue, return TRUE is state is changing
    fn try_set_continued(&mut self) -> bool {
        if self.state == JobState::Stoped {
            self.state = JobState::Continued;
            self.last_event = Some(JobState::Continued);
            true
        } else {
            false
        }
    }
    /// Try to set as stoped, return TRUE is state is changing
    fn try_set_stoped(&mut self) -> bool {
        if self.state == JobState::Continued {
            self.state = JobState::Stoped;
            self.last_event = Some(JobState::Stoped);
            true
        } else {
            false
        }
    }
    /// Usable method for waitpid for exemple
    pub fn consume_last_event(&mut self) -> Option<JobState> {
        let evt = self.last_event;
        self.last_event = None;
        evt
    }
}
