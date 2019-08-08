#![cfg_attr(not(test), no_std)]
//! Message to send to the scheduler
extern crate alloc;
use lazy_static::lazy_static;
// use super::scheduler::Pid;
use alloc::collections::vec_deque::VecDeque;
use libc_binding::{Pid, Signum};
use sync::DeadMutex;

// /// message for the tty driver
// #[derive(Debug, Copy, Clone)]
// pub enum TtyMessage {
//     /// which key was press
//     KeyPress { keysymb: KeySymb },
// }

/// message for the scheduler
#[derive(Debug, Copy, Clone)]
pub enum SchedulerMessage {
    /// there is something to read
    SomethingToRead,
}

/// message for a process
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProcessMessage {
    /// this process has died
    ProcessDied { pid: Pid },
    /// there is something to read
    SomethingToRead,
}

/// enum MessageTo contains the receiver in its variant and the
/// message in its variant content
#[derive(Debug, Copy, Clone)]
pub enum MessageTo {
    // Tty { content: TtyMessage },
    Process { pid: Pid, content: ProcessMessage },
    ProcessGroup { pgid: Pid, content: Signum },
    Scheduler { content: SchedulerMessage },
}

#[derive(Debug)]
pub struct MessageQueue<T> {
    list: VecDeque<T>,
}

impl<T> MessageQueue<T> {
    pub fn new() -> Self {
        Self {
            list: VecDeque::new(),
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }
    pub fn push_back(&mut self, message: T) {
        self.list.push_back(message)
    }
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.list.retain(f);
    }
}

lazy_static! {
    /// Global Message queue of the kernel, Written by anybody and
    /// readen by the scheduler which can dispatch the message afterwards
    static ref MESSAGE_QUEUE: DeadMutex<MessageQueue<MessageTo>> =
        DeadMutex::new(MessageQueue::new());
}

/// push a message on to the global MESSAGE_QUEUE
pub fn push_message(message: MessageTo) {
    MESSAGE_QUEUE.lock().push_back(message)
}

/// pop a message from the global MESSAGE_QUEUE
pub fn pop_message() -> Option<MessageTo> {
    MESSAGE_QUEUE.lock().pop_front()
}
