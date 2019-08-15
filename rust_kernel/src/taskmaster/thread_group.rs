use super::scheduler::{Pid, Tid};
use super::syscall::clone::CloneFlags;
use super::task::Task;
use super::SysResult;
use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use core::ffi::c_void;
use fallible_collections::{btree::BTreeMap, FallibleVec, TryClone};
use libc_binding::{gid_t, uid_t};
use try_clone_derive::TryClone;

#[derive(Debug)]
pub enum ThreadGroupState {
    Running {
        all_thread: ThreadList,
    },
    /// The process is terminated and wait to deliver his testament to his father
    // TODO: Use bits 0..7 for normal exit(). Interpreted as i8 and set bit 31
    // TODO: Use bits 8..15 for signal exit. Interpreted as i8 and set bit 30
    // Zombie(i32),
    Zombie(i32),
}

type ThreadList = BTreeMap<Tid, Task>;

impl ThreadGroupState {
    pub fn get_thread_list_mut(&mut self) -> Option<&mut ThreadList> {
        match self {
            Self::Running { all_thread } => Some(all_thread),
            Self::Zombie(_) => None,
        }
    }

    pub fn get_thread_list(&self) -> Option<&ThreadList> {
        match self {
            Self::Running { all_thread } => Some(all_thread),
            Self::Zombie(_) => None,
        }
    }

    fn get_death_status(&self) -> Option<i32> {
        match self {
            Self::Zombie(status) => Some(*status),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ThreadGroup {
    /// the identity(uid, gid, groups...)
    pub credentials: Credentials,
    /// all the thread in the thread group
    pub thread_group_state: ThreadGroupState,
    /// the process group id
    pub pgid: Pid,
    /// List of childs
    pub child: Vec<Pid>,
    /// Parent
    pub parent: Option<Pid>,
    /// the next availabel tid for a new thread
    next_tid: Tid,
}

#[derive(Debug, TryClone)]
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
    pub fn try_new(
        father_pid: Option<Pid>,
        task: Task,
        pgid: Pid,
    ) -> Result<Self, CollectionAllocErr> {
        let mut all_thread = BTreeMap::new();
        all_thread.try_insert(0, task)?;
        Ok(ThreadGroup {
            child: Vec::new(),
            parent: father_pid,
            credentials: Credentials::ROOT,
            thread_group_state: ThreadGroupState::Running {
                all_thread: all_thread,
            },
            next_tid: 1,
            pgid,
        })
    }

    pub fn get_first_thread(&mut self) -> Option<&mut Task> {
        self.get_all_thread_mut()?.get_mut(&0)
    }

    pub fn get_thread(&mut self, thread_id: Tid) -> Option<&mut Task> {
        self.get_all_thread_mut()?.get_mut(&thread_id)
    }

    pub fn get_all_thread(&self) -> Option<&ThreadList> {
        self.thread_group_state.get_thread_list()
    }

    pub fn get_all_thread_mut(&mut self) -> Option<&mut ThreadList> {
        self.thread_group_state.get_thread_list_mut()
    }

    pub fn get_available_tid(&mut self) -> Tid {
        let res = self.next_tid;
        self.next_tid += 1;
        res
    }

    pub fn get_death_status(&self) -> Option<i32> {
        self.thread_group_state.get_death_status()
    }

    pub fn is_zombie(&self) -> bool {
        self.get_death_status().is_some()
    }

    pub fn sys_clone(
        &mut self,
        father_pid: Pid,
        father_tid: Tid,
        child_pid: Pid,
        kernel_esp: u32,
        child_stack: *const c_void,
        flags: CloneFlags,
    ) -> SysResult<Self> {
        // TODO: if new_thread_group fail remove that
        self.child.try_push(child_pid)?;

        let new_task = self
            .get_thread(father_tid)
            .expect("no father tid wtf")
            .sys_clone(kernel_esp, child_stack, flags)?;

        let mut all_thread = BTreeMap::new();
        all_thread.try_insert(0, new_task)?;
        Ok(Self {
            child: Vec::new(),
            parent: Some(father_pid),
            credentials: self.credentials.try_clone()?,
            thread_group_state: ThreadGroupState::Running {
                all_thread: all_thread,
            },
            pgid: self.pgid,
            next_tid: 1,
        })
    }

    /// remove pid `pid` from the child list, Panic if not present
    pub fn remove_child(&mut self, pid: Pid) {
        self.child
            .remove_item(&pid)
            .expect("can't remove child pid it is not present");
    }
}
