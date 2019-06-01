use super::Pid;
use crate::taskmaster::process::Process;
use alloc::vec::Vec;
use core::mem;

#[derive(Debug)]
pub struct Task {
    pub process_state: ProcessState,
    pub child: Vec<Pid>,
    pub parent: Option<Pid>,
}

impl Task {
    pub fn new(parent: Option<Pid>, process_state: ProcessState) -> Self {
        Self { process_state, child: Vec::new(), parent }
    }

    pub fn unwrap_running_mut(&mut self) -> &mut Process {
        match &mut self.process_state {
            ProcessState::Waiting(process) | ProcessState::Running(process) => process,
            _ => panic!("WTF"),
        }
    }

    pub fn unwrap_running(&self) -> &Process {
        match &self.process_state {
            ProcessState::Running(process) => process,
            _ => panic!("WTF"),
        }
    }

    pub fn is_zombie(&self) -> bool {
        match self.process_state {
            ProcessState::Zombie(_) => true,
            _ => false,
        }
    }

    pub fn is_waiting(&self) -> bool {
        match self.process_state {
            ProcessState::Waiting(_) => true,
            _ => false,
        }
    }

    pub fn set_waiting(&mut self) {
        let uninit = unsafe { mem::uninitialized() };
        let prev = mem::replace(&mut self.process_state, uninit);
        let next = prev.set_waiting();
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
pub enum ProcessState {
    /// The process is currently on running state
    Running(Process),
    /// The process is currently waiting for the die of its childrens
    Waiting(Process),
    /// The process is terminated and wait to deliver his testament to his father
    Zombie(i32),
}

impl ProcessState {
    pub fn set_waiting(self) -> Self {
        match self {
            ProcessState::Running(p) => ProcessState::Waiting(p),
            _ => panic!("already waiting"),
        }
    }
    pub fn set_running(self) -> Self {
        match self {
            ProcessState::Waiting(p) => ProcessState::Running(p),
            _ => panic!("already running"),
        }
    }
}
