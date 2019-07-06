use super::scheduler::Tid;
use super::task::Task;
use alloc::collections::CollectionAllocErr;
use hashmap_core::fnv::FnvHashMap as HashMap;

#[derive(Debug)]
pub struct ThreadGroup {
    next_tid: Tid,
    pub all_thread: HashMap<Tid, Task>,
}

impl ThreadGroup {
    pub fn try_new(task: Task) -> Result<Self, CollectionAllocErr> {
        let mut all_thread = HashMap::new();
        all_thread.try_reserve(1)?;
        all_thread.insert(0, task);
        Ok(ThreadGroup {
            all_thread,
            next_tid: 1,
        })
    }

    pub fn get_available_tid(&mut self) -> Tid {
        let res = self.next_tid;
        self.next_tid += 1;
        res
    }
}
