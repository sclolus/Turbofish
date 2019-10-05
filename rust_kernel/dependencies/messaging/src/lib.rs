#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::collections::vec_deque::VecDeque;
use keyboard::{KeyCode, KeySymb, ScanCode};
use libc_binding::{Pid, Signum};

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
        scancode: ScanCode,
        keycode: Option<KeyCode>,
        keysymb: Option<KeySymb>,
    },
    Accepter {
        uid_file_op: usize,
    },
    Connecter {
        uid_file_op: usize,
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

mod scheduler {
    use super::MessageTo;
    extern "C" {
        #[allow(improper_ctypes)]
        pub fn send_message(message: MessageTo);
    }
}

pub unsafe fn send_message(message: MessageTo) {
    // call with the linker the send message function of the scheduler
    scheduler::send_message(message);
}
