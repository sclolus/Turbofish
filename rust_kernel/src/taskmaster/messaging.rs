//! Message to send to the scheduler
use super::scheduler::Pid;
use alloc::collections::vec_deque::VecDeque;
use sync::DeadMutex;

#[derive(Debug, Copy, Clone)]
pub struct Message {
    dest: Pid,
    message_content: MessageContent,
}

impl Message {
    pub fn new(dest: Pid, message_content: MessageContent) -> Self {
        Self {
            dest,
            message_content,
        }
    }
    pub fn get_content(&self) -> MessageContent {
        self.message_content
    }
    pub fn get_dest(&self) -> Pid {
        self.dest
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MessageContent {
    ProcessDied { pid: Pid },
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
}

lazy_static! {
    pub static ref MESSAGE_QUEUE: DeadMutex<MessageQueue<Message>> =
        DeadMutex::new(MessageQueue::new());
}
