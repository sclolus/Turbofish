use super::scheduler::{Pid, Tid};
use super::task::Task;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use hashmap_core::fnv::FnvHashMap as HashMap;
use libc_binding::{gid_t, uid_t};

#[derive(Debug)]
pub struct ThreadGroup {
    pub credentials: Credentials,
    next_tid: Tid,
    pub all_thread: HashMap<Tid, Task>,
    pub pgid: Pid,
}

#[derive(Debug)]
pub struct Credentials {
    pub uid: uid_t,
    pub gid: gid_t,
    pub euid: uid_t,
    pub egid: gid_t,
    pub suid: uid_t,
    pub sgid: gid_t,
    pub groups: Vec<gid_t>,
}

impl Credentials {
    const ROOT: Self = Self {
        uid: 0,
        gid: 0,
        euid: 0,
        egid: 0,
        suid: 0,
        sgid: 0,
        groups: Vec::new(),
    };
}

impl ThreadGroup {
    pub fn try_new(task: Task, pgid: Pid) -> Result<Self, CollectionAllocErr> {
        let mut all_thread = HashMap::new();
        all_thread.try_reserve(1)?;
        all_thread.insert(0, task);
        Ok(ThreadGroup {
            credentials: Credentials::ROOT,
            all_thread,
            next_tid: 1,
            pgid,
        })
    }

    pub fn get_first_thread(&mut self) -> &mut Task {
        self.all_thread
            .get_mut(&0)
            .expect("thread 0 should be there")
    }

    pub fn get_available_tid(&mut self) -> Tid {
        let res = self.next_tid;
        self.next_tid += 1;
        res
    }
}
