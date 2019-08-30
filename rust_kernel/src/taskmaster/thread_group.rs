use super::fd_interface::FileDescriptorInterface;
use super::scheduler::{Pid, Tid};
use super::syscall::clone::CloneFlags;
use super::thread::Thread;
use super::SysResult;

use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;
use core::ffi::c_void;
use fallible_collections::{btree::BTreeMap, FallibleVec, TryClone};
use libc_binding::{gid_t, uid_t, Signum};
use try_clone_derive::TryClone;

#[derive(Debug)]
pub enum ThreadGroupState {
    /// The process is running and has a thread list
    Running(RunningThreadGroup),
    /// The process is terminated and wait to deliver his testament to his father
    /// bits 0..7 for normal exit(). Interpreted as i8 and set bit 31
    /// bits 8..15 for signal exit. Interpreted as i8 and set bit 30
    Zombie(Status),
}

/// Main boilerplate
#[derive(Debug)]
pub struct RunningThreadGroup {
    all_thread: ThreadList,
    /// List of childs
    pub child: Vec<Pid>,
    /// File Descriptors
    pub file_descriptor_interface: FileDescriptorInterface,
}

type ThreadList = BTreeMap<Tid, Thread>;

impl ThreadGroupState {
    fn get_death_status(&self) -> Option<Status> {
        match self {
            Self::Zombie(status) => Some(*status),
            _ => None,
        }
    }

    pub fn get_thread_list(&self) -> Option<&ThreadList> {
        match self {
            Self::Running(running_thread_group) => Some(&running_thread_group.all_thread),
            Self::Zombie(_) => None,
        }
    }

    pub fn get_thread_list_mut(&mut self) -> Option<&mut ThreadList> {
        match self {
            Self::Running(running_thread_group) => Some(&mut running_thread_group.all_thread),
            Self::Zombie(_) => None,
        }
    }

    pub fn unwrap_running(&self) -> &RunningThreadGroup {
        match self {
            Self::Running(running_thread_group) => running_thread_group,
            Self::Zombie(_) => panic!("Cannot unwrap a zombie !"),
        }
    }

    pub fn unwrap_running_mut(&mut self) -> &mut RunningThreadGroup {
        match self {
            Self::Running(running_thread_group) => running_thread_group,
            Self::Zombie(_) => panic!("Cannot unwrap a zombie !"),
        }
    }
}

#[derive(Debug)]
pub struct ThreadGroup {
    /// the identity(uid, gid, groups...)
    pub credentials: Credentials,
    /// all the thread in the thread group
    thread_group_state: ThreadGroupState,
    /// the process group id
    pub pgid: Pid,
    /// Parent
    pub parent: Pid,
    /// the next availabel tid for a new thread
    next_tid: Tid,
    /// Current job status of a process
    pub job: Job,
}

#[derive(Debug, TryClone)]
/// all the identity associate to a thread group
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
    /// the Credential of the ROOT user
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
    pub fn try_new(father_pid: Pid, thread: Thread, pgid: Pid) -> Result<Self, CollectionAllocErr> {
        let mut all_thread = BTreeMap::new();
        all_thread.try_insert(0, thread)?;
        Ok(ThreadGroup {
            parent: father_pid,
            credentials: Credentials::ROOT,
            thread_group_state: ThreadGroupState::Running(RunningThreadGroup {
                all_thread: all_thread,
                child: Vec::new(),
                file_descriptor_interface: FileDescriptorInterface::new(),
            }),
            next_tid: 1,
            pgid,
            job: Job::new(),
        })
    }

    pub fn get_available_tid(&mut self) -> Tid {
        let res = self.next_tid;
        self.next_tid += 1;
        res
    }

    pub fn get_death_status(&self) -> Option<Status> {
        self.thread_group_state.get_death_status()
    }

    pub fn is_zombie(&self) -> bool {
        self.get_death_status().is_some()
    }

    /// Clone a thread group
    /// the clone was called from thread father_tid
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
        self.unwrap_running_mut().child.try_push(child_pid)?;

        let new_thread = self
            .get_thread(father_tid)
            .expect("no father tid wtf")
            .sys_clone(kernel_esp, child_stack, flags)?;

        let mut all_thread = BTreeMap::new();
        all_thread.try_insert(0, new_thread)?;
        Ok(Self {
            parent: father_pid,
            credentials: self.credentials.try_clone()?,
            thread_group_state: ThreadGroupState::Running(RunningThreadGroup {
                all_thread: all_thread,
                child: Vec::new(),
                file_descriptor_interface: self
                    .unwrap_running()
                    .file_descriptor_interface
                    .try_clone()?,
            }),
            pgid: self.pgid,
            next_tid: 1,
            job: Job::new(),
        })
    }

    /// remove pid `pid` from the child list, Panic if not present
    pub fn remove_child(&mut self, pid: Pid) {
        self.unwrap_running_mut()
            .child
            .remove_item(&pid)
            .expect("can't remove child pid it is not present");
    }

    pub fn set_zombie(&mut self, status: Status) {
        self.thread_group_state = ThreadGroupState::Zombie(status);
    }

    pub fn iter_thread_mut(&mut self) -> impl Iterator<Item = &mut Thread> {
        self.get_all_thread_mut()
            .into_iter()
            .flat_map(|all_thread| all_thread.values_mut())
    }

    pub fn get_first_thread(&mut self) -> Option<&mut Thread> {
        self.get_all_thread_mut()?.get_mut(&0)
    }

    pub fn get_thread(&mut self, thread_id: Tid) -> Option<&mut Thread> {
        self.get_all_thread_mut()?.get_mut(&thread_id)
    }

    pub fn get_all_thread(&self) -> Option<&ThreadList> {
        self.thread_group_state.get_thread_list()
    }

    pub fn get_all_thread_mut(&mut self) -> Option<&mut ThreadList> {
        self.thread_group_state.get_thread_list_mut()
    }

    /// Unwrap directly the field thread_group_state as Running
    pub fn unwrap_running(&self) -> &RunningThreadGroup {
        self.thread_group_state.unwrap_running()
    }

    /// Unwrap directly the field thread_group_state as Running
    pub fn unwrap_running_mut(&mut self) -> &mut RunningThreadGroup {
        self.thread_group_state.unwrap_running_mut()
    }
}

/// Global design of User Program Status
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Exited(i32),
    Signaled(Signum),
    Stopped,
    Continued,
}

impl Status {
    pub fn is_exited(&self) -> bool {
        match self {
            Self::Exited(_) => true,
            _ => false,
        }
    }
    pub fn is_terminated(&self) -> bool {
        match self {
            Self::Exited(_) | Self::Signaled(_) => true,
            _ => false,
        }
    }
    pub fn is_signaled(&self) -> bool {
        match self {
            Self::Signaled(_) => true,
            _ => false,
        }
    }
}

impl From<JobState> for Status {
    fn from(job_state: JobState) -> Self {
        match job_state {
            JobState::Continued => Self::Continued,
            JobState::Stopped => Self::Stopped,
        }
    }
}

use libc_binding::{
    CONTINUED_STATUS_BIT, EXITED_STATUS_BITS, SIGNALED_STATUS_BITS, SIGNALED_STATUS_SHIFT,
    STOPPED_STATUS_BIT,
};

/// Boilerlate
impl From<Status> for i32 {
    fn from(status: Status) -> Self {
        use Status::*;
        match status {
            Exited(v) => v,
            Signaled(signum) => (signum as i32) << SIGNALED_STATUS_SHIFT as i32,
            Stopped => STOPPED_STATUS_BIT as _,
            Continued => CONTINUED_STATUS_BIT as _,
        }
    }
}

/// Another boilerplate
impl From<i32> for Status {
    fn from(status: i32) -> Self {
        use Status::*;
        if status & !EXITED_STATUS_BITS as i32 == 0 {
            Exited(status)
        } else if status & EXITED_STATUS_BITS as i32 == 0
            && status & !SIGNALED_STATUS_BITS as i32 == 0
        {
            Signaled(unsafe { core::mem::transmute(status >> SIGNALED_STATUS_SHIFT) })
        } else if status & !STOPPED_STATUS_BIT as i32 == 0 {
            Stopped
        } else if status & !CONTINUED_STATUS_BIT as i32 == 0 {
            Continued
        } else {
            panic!("Status is Bullshit !");
        }
    }
}

/// State of a process in the point of view of JobAction
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum JobState {
    Stopped,
    Continued,
}

/// Mais Job structure
#[derive(Debug)]
pub struct Job {
    /// Current JobState
    state: JobState,
    /// Last change state (this event may be consumed by waitpid)
    last_event: Option<JobState>,
}

/// Main Job implementation
impl Job {
    const fn new() -> Self {
        Self {
            state: JobState::Continued,
            last_event: None,
        }
    }
    /// Try to set as continue, return TRUE is state is changing
    pub fn try_set_continued(&mut self) -> bool {
        if self.state == JobState::Stopped {
            self.state = JobState::Continued;
            self.last_event = Some(JobState::Continued);
            true
        } else {
            false
        }
    }
    /// Try to set as stoped, return TRUE is state is changing
    pub fn try_set_stoped(&mut self) -> bool {
        if self.state == JobState::Continued {
            self.state = JobState::Stopped;
            self.last_event = Some(JobState::Stopped);
            true
        } else {
            false
        }
    }
    /// Usable method for waitpid for exemple
    pub fn consume_last_event(&mut self) -> Option<JobState> {
        self.last_event.take()
    }

    /// get the last event
    pub fn get_last_event(&self) -> Option<JobState> {
        self.last_event
    }
}
