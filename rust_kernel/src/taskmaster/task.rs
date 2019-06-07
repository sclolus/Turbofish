//! This file contains definition of a task

use super::process::UserProcess;
use super::scheduler::Pid;
use super::signal::SignalInterface;

use alloc::boxed::Box;
use alloc::vec::Vec;

use core::mem;

#[derive(Debug)]
pub struct Task {
    pub process_state: ProcessState,
    pub child: Vec<Pid>,
    pub parent: Option<Pid>,
    pub signal: SignalInterface,
}

impl Task {
    pub fn new(parent: Option<Pid>, process_state: ProcessState) -> Self {
        Self { process_state, child: Vec::new(), parent, signal: SignalInterface::new() }
    }

    pub fn unwrap_running_mut(&mut self) -> &mut UserProcess {
        match &mut self.process_state {
            ProcessState::Waiting(process, _) | ProcessState::Running(process) => process,
            _ => panic!("WTF"),
        }
    }

    pub fn unwrap_running(&self) -> &UserProcess {
        match &self.process_state {
            ProcessState::Running(process) | ProcessState::Waiting(process, _) => process,
            _ => panic!("WTF"),
        }
    }

    pub fn is_zombie(&self) -> bool {
        match self.process_state {
            ProcessState::Zombie(_) => true,
            _ => false,
        }
    }

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

#[derive(Debug)]
pub enum WaitingState {
    /// The Process is sleeping until pit time >= u32 value
    Sleeping(u32),
    /// The Process is looking for the death of his child
    /// Set none for undefined PID or a child PID. Is followed by the status field
    ChildDeath(Option<Pid>, u32),
}

#[derive(Debug)]
pub enum ProcessState {
    /// The process is currently on running state
    Running(Box<UserProcess>),
    /// The process is currently waiting for something
    Waiting(Box<UserProcess>, WaitingState),
    /// The process is terminated and wait to deliver his testament to his father
    /// The process is terminated and wait to deliver his testament to his father
    // TODO: Use bits 0..7 for normal exit(). Interpreted as i8 and set bit 31
    // TODO: Use bits 8..15 for signal exit. Interpreted as i8 and set bit 30
    Zombie(i32),
}

impl ProcessState {
    pub fn set_waiting(self, waiting_state: WaitingState) -> Self {
        match self {
            ProcessState::Running(p) => ProcessState::Waiting(p, waiting_state),
            ProcessState::Waiting(p, _) => ProcessState::Waiting(p, waiting_state),
            _ => panic!("Not handled by this feature"),
        }
    }
    pub fn set_running(self) -> Self {
        match self {
            ProcessState::Waiting(p, _) => ProcessState::Running(p),
            _ => panic!("already running"),
        }
    }
}
