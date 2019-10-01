use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Errno, Pid, Whence};

#[derive(Debug, Clone)]
pub struct StatDriver {
    inode_id: InodeId,
    pid: Pid,
}

impl StatDriver {
    pub fn new(inode_id: InodeId, pid: Pid) -> Self {
        Self { inode_id, pid }
    }
}

unsafe impl Send for StatDriver {}

#[derive(Debug)]
pub struct StatOperations {
    inode_id: InodeId,
    pid: Pid,
    offset: usize,
}

impl Driver for StatDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(StatOperations::new(self.inode_id, self.pid, 0)))?;
        Ok(IpcResult::Done(res))
    }
}

impl StatOperations {
    pub fn new(inode_id: InodeId, pid: Pid, offset: usize) -> Self {
        Self {
            inode_id,
            pid,
            offset,
        }
    }
}

impl ProcFsOperations for StatOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        // TODO: This is dummy.
        let stat_string = format!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n", self.pid,
                                  // comm
                                  0,
                                  // state
                                  0,
                                  // ppid
                                  0,
                                  // pgrp
                                  0,
                                  // session
                                  0,
                                  // tty_nr
                                  0,
                                  // tpgid
                                  0,
                                  // flags
                                  0,
                                  // minflt
                                  0,
                                  // cminflt
                                  0,
                                  // majflt
                                  0,
                                  // cmajflt
                                  0,
                                  // utime
                                  42,
                                  // stime
                                  42,
                                  // cutime
                                  42,
                                  // cstime
                                  42,
                                  // priority
                                  0,
                                  // nice
                                  0,
                                  // num_threads
                                  0,
                                  // itrealvalue
                                  0,
                                  // starttime
                                  0,
                                  // vsize
                                  0,
                                  // rss
                                  0,
                                  // rsslim
                                  0,
                                  // startcode
                                  0,
                                  // endcode
                                  0,
                                  // startstack
                                  0,
                                  // kstkesp
                                  0,
                                  // kstkeip
                                  0,
                                  // signal
                                  0,
                                  // blocked
                                  0,
                                  // sigignore
                                  0,
                                  // sigcatch
                                  0,
                                  // wchan
                                  0,
                                  // nswap
                                  0,
                                  // cnswap
                                  0,
                                  // exit_signal
                                  0,
                                  // processor
                                  0,
                                  // rt_priority
                                  0,
                                  // policy
                                  0,
                                  // delayacct_blkio_ticks
                                  0,
                                  // guest_time
                                  0,
                                  // cguest_time
                                  0,
                                  // start_data
                                  0,
                                  // end_data
                                  0,
                                  // start_brk
                                  0,
                                  // arg_start
                                  0,
                                  // arg_end
                                  0,
                                  // env_start
                                  0,
                                  // env_end
                                  0,
                                  // exit_code
                                  0,
        );
        Ok(Cow::from(stat_string))
    }
}

impl FileOperation for StatOperations {
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

impl Drop for StatOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
