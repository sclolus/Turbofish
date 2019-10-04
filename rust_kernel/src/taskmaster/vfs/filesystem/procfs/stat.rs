use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};
use crate::taskmaster::SCHEDULER;

use crate::taskmaster::scheduler::ThreadGroupState;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use fallible_collections::{FallibleArc, TryCollect};

use libc_binding::{Errno, OpenFlags};
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Pid, Whence};

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
        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock(); //code duplication with comm.rs
        let thread_group = scheduler
            .get_thread_group(self.pid)
            .expect("CommOperations::read(): The Process should exist");

        let comm = {
            match thread_group.argv.as_ref() {
                Some(comm) => comm,
                None => return Ok(Cow::from("")),
            }
        };

        let mut bytes: Vec<u8> = comm
            .strings()
            .next()
            .iter()
            .flat_map(|s| s.iter().map(|b| *b as u8))
            .filter(|c| *c != '\0' as u8)
            .try_collect()?;

        let comm = String::from_utf8(bytes).map_err(|_| {
            log::error!("invalid utf8 in environ operation");
            Errno::EINVAL
        })?;

        let state = match thread_group.thread_group_state {
            ThreadGroupState::Running(_) => "R",
            ThreadGroupState::Zombie(_status) => "Z",
        };

        let utime = thread_group.process_duration.user_time().as_secs(); // convert to clock tick count.
        let stime = thread_group.process_duration.system_time().as_secs();

        let stat_string = tryformat!(4096, "{} ({}) {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n", self.pid,
                                  // comm
                                  comm,
                                  // state
                                  state,
                                  // ppid
                                  thread_group.parent,
                                  // pgrp
                                  thread_group.pgid,
                                  // session
                                  1,
                                  // tty_nr
                                  4 << 8 | 1, // TODO: get the real controlling terminal.
                                  // tpgid
                                  1,
                                  // flags
                                  1,
                                  // minflt
                                  1,
                                  // cminflt
                                  1,
                                  // majflt
                                  1,
                                  // cmajflt
                                  1,
                                  // utime
                                  utime,
                                  // stime
                                  stime,
                                  // cutime
                                  42,
                                  // cstime
                                  42,
                                  // priority
                                  1,
                                  // nice
                                  1,
                                  // num_threads
                                  1,
                                  // itrealvalue
                                  1,
                                  // starttime
                                  1,
                                  // vsize
                                  1,
                                  // rss
                                  1,
                                  // rsslim
                                  1,
                                  // startcode
                                  1,
                                  // endcode
                                  1,
                                  // startstack
                                  1,
                                  // kstkesp
                                  1,
                                  // kstkeip
                                  1,
                                  // signal
                                  1,
                                  // blocked
                                  1,
                                  // sigignore
                                  1,
                                  // sigcatch
                                  1,
                                  // wchan
                                  1,
                                  // nswap
                                  1,
                                  // cnswap
                                  1,
                                  // exit_signal
                                  1,
                                  // processor
                                  1,
                                  // rt_priority
                                  1,
                                  // policy
                                  1,
                                  // delayacct_blkio_ticks
                                  1,
                                  // guest_time
                                  1,
                                  // cguest_time
                                  1,
                                  // start_data
                                  1,
                                  // end_data
                                  1,
                                  // start_brk
                                  1,
                                  // arg_start
                                  1,
                                  // arg_end
                                  1,
                                  // env_start
                                  1,
                                  // env_end
                                  1,
                                  // exit_code
                                  0,
        )?;
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
