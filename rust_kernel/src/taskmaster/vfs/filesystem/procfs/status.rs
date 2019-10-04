use super::{
    Driver, FileOperation, InodeId, IpcResult, Path, ProcFsOperations, SysResult, PATH_MAX, VFS,
};
use crate::taskmaster::SCHEDULER;

use crate::taskmaster::scheduler::ThreadGroupState;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;

use fallible_collections::{FallibleArc, TryCollect};

use libc_binding::{Errno, OpenFlags};
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Pid, Whence};

#[derive(Debug, Clone)]
pub struct StatusDriver {
    inode_id: InodeId,
    pid: Pid,
}

impl StatusDriver {
    pub fn new(inode_id: InodeId, pid: Pid) -> Self {
        Self { inode_id, pid }
    }
}

unsafe impl Send for StatusDriver {}

#[derive(Debug)]
pub struct StatusOperations {
    inode_id: InodeId,
    pid: Pid,
    offset: usize,
}

impl Driver for StatusDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(StatusOperations::new(
            self.inode_id,
            self.pid,
            0,
        )))?;
        Ok(IpcResult::Done(res))
    }
}

impl StatusOperations {
    pub fn new(inode_id: InodeId, pid: Pid, offset: usize) -> Self {
        Self {
            inode_id,
            pid,
            offset,
        }
    }
}

impl ProcFsOperations for StatusOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock(); //code duplication with comm.rs
        let thread_group = scheduler
            .get_thread_group(self.pid)
            .expect("CommOperations::read(): The Process should exist");

        let default_name = Path::try_from("-")?;
        let name = tryformat!(
            PATH_MAX,
            "{}",
            thread_group
                .filename
                .as_ref()
                .unwrap_or(&default_name)
                .filename()
                .unwrap()
        )?;

        let pid = self.pid;

        let umask = thread_group.umask;
        let (state, long_state) = match thread_group.thread_group_state {
            ThreadGroupState::Running(_) => ("R", "running"),
            ThreadGroupState::Zombie(_status) => ("Z", "zombie"),
        };

        let uid = thread_group.credentials.uid;
        let euid = thread_group.credentials.euid;
        let suid = thread_group.credentials.suid;

        let f_uid = 0; // Dunno about that.

        let gid = thread_group.credentials.gid;
        let egid = thread_group.credentials.egid;
        let sgid = thread_group.credentials.sgid;

        let g_f_uid = 0; // Dunno about that.

        let status_string = tryformat!(
            256,
            "Name:	{}\n\
             Umask:	{}\n\
             State:	{} ({})\n\
             Tgid:	0\n\
             Ngid:	0\n\
             Pid:	{}\n\
             PPid:	{}\n\
             TracerPid:	0\n\
             Uid:	{}	{}	{}	{}\n\
             Gid:	{}	{}	{}	{}\n\
             FDSize:	64\n\
             Groups:	 \n\
             NStgid:	42\n\
             NSpid:	42\n\
             NSpgid:	42\n\
             NSsid:	42\n\
             VmPeak:     131168 kB\n\
             VmSize:     131168 kB\n\
             VmLck:           0 kB\n\
             VmPin:           0 kB\n\
             VmHWM:       13484 kB\n\
             VmRSS:       13484 kB\n\
             RssAnon:     10264 kB\n\
             RssFile:      3220 kB\n\
             RssShmem:        0 kB\n\
             VmData:      10332 kB\n\
             VmStk:         136 kB\n\
             VmExe:         992 kB\n\
             VmLib:        2104 kB\n\
             VmPTE:          76 kB\n\
             VmPMD:          12 kB\n\
             VmSwap:          0 kB\n\
             Threads:	1\n\
             SigQ:	0/30562\n\
             SigPnd:	0000000000000000\n\
             ShdPnd:	0000000000000000\n\
             SigBlk:	0000000000000000\n\
             SigIgn:	ffffffffffffffff\n\
             SigCgt:	0000000000000000\n\
             CapInh:	0000000000000000\n\
             CapPrm:	0000003fffffffff\n\
             CapEff:	0000003fffffffff\n\
             CapBnd:	0000003fffffffff\n\
             CapAmb:	0000000000000000\n\
             NoNewPrivs:	0\n\
             Seccomp:	0\n\
             Speculation_Store_Bypass:	vulnerable\n\
             Cpus_allowed:	1\n\
             Cpus_allowed_list:	0\n\
             Mems_allowed:	00000000,00000001\n\
             Mems_allowed_list:	0\n\
             voluntary_ctxt_switches:	0\n\
             nonvoluntary_ctxt_switches:	0\n",
            name,
            umask,
            state,
            long_state,
            pid,
            thread_group.parent,
            uid,
            euid,
            suid,
            f_uid,
            gid,
            egid,
            sgid,
            g_f_uid,
        )?;
        Ok(Cow::from(status_string))
    }
}

impl FileOperation for StatusOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.seq_read(buf)
    }

    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        self.proc_lseek(offset, whence)
    }
}

impl Drop for StatusOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
