//! Message to send to the scheduler
use super::scheduler::Pid;
use alloc::collections::vec_deque::VecDeque;
use sync::DeadMutex;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    ProcessDied { pid: Pid },
}

pub struct MessageQueue {
    list: VecDeque<Message>,
}

impl MessageQueue {
    fn new() -> Self {
        Self {
            list: VecDeque::new(),
        }
    }
    pub fn pop_front(&mut self) -> Option<Message> {
        self.list.pop_front()
    }
    pub fn push_back(&mut self, message: Message) {
        self.list.push_back(message)
    }
}

lazy_static! {
    pub static ref MESSAGE_QUEUE: DeadMutex<MessageQueue> = DeadMutex::new(MessageQueue::new());
}
