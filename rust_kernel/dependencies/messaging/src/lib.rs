#![cfg_attr(not(test), no_std)]
//! Message to send to the scheduler
extern crate alloc;
use lazy_static::lazy_static;
// use super::scheduler::Pid;
use alloc::collections::vec_deque::VecDeque;
use keyboard::keysymb::KeySymb;
use libc_binding::{Pid, Signum};
use sync::LockForest;

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
    /// this process has died, or continued, or stopped
    ProcessUpdated { pid: Pid, pgid: Pid, status: i32 },
    /// there is something to read
    SomethingToRead,
    /// there is something to write
    SomethingToWrite,
    /// there is something to open
    SomethingToOpen,
}

#[derive(Debug, Copy, Clone)]
pub enum ProcessGroupMessage {
    Signal(Signum),
    SomethingToRead,
}

/// enum MessageTo contains the receiver in its variant and the
/// message in its variant content
#[derive(Debug, Copy, Clone)]
pub enum MessageTo {
    // Tty { content: TtyMessage },
    /// IPC: Adressed to a specific reader
    Reader {
        uid_file_op: usize,
    },
    /// IPC: Adressed to a specific writer
    Writer {
        uid_file_op: usize,
    },
    /// IPC: Adressed to a specific opener
    Opener {
        uid_file_op: usize,
    },
    Process {
        pid: Pid,
        content: ProcessMessage,
    },
    ProcessGroup {
        pgid: Pid,
        content: ProcessGroupMessage,
    },
    Scheduler {
        content: SchedulerMessage,
    },
    Tty {
        key_pressed: KeySymb,
    },
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

/// message queue can contain 50 messages
const MESSAGE_QUEUE_CAPACITY: usize = 50;

lazy_static! {
    /// Global Message queue of the kernel, Written by anybody and
    /// readen by the scheduler which can dispatch the message afterwards
    static ref MESSAGE_QUEUE: LockForest<MessageTo> = LockForest::new(MESSAGE_QUEUE_CAPACITY);
}

/// push a message on to the global MESSAGE_QUEUE
pub fn push_message(message: MessageTo) {
    // TODO: remove this expect one day
    MESSAGE_QUEUE.push(message).expect("message queue full");
}

pub fn drain_messages() -> impl Iterator<Item = MessageTo> {
    MESSAGE_QUEUE.drain()
}

mod scheduler {
    use super::MessageTo;
    extern "C" {
        #[allow(improper_ctypes)]
        pub fn send_message(message: MessageTo);
    }
}

pub fn send_message(message: MessageTo) {
    unsafe {
        // call with the linker the send message function of the scheduler
        scheduler::send_message(message);
    }
}
