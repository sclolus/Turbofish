use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{Errno, Pid};

#[derive(Debug, Clone)]
pub struct StatDriver {
    pid: Pid,
}

unsafe impl Send for StatDriver {}

#[derive(Debug)]
pub struct StatOperations {
    pid: Pid,
    offset: usize,
}

impl Driver for StatDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(StatOperations::new(self.pid, 0)))?;
        Ok(IpcResult::Done(res))
    }
}

impl StatDriver {
    pub fn new(pid: Pid) -> Self {
        Self { pid }
    }
}

impl StatOperations {
    pub fn new(pid: Pid, offset: usize) -> Self {
        Self { pid, offset }
    }
}

impl FileOperation for StatOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

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
                                  0,
                                  // stime
                                  0,
                                  // cutime
                                  0,
                                  // cstime
                                  0,
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
        if self.offset >= stat_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let stat_string = &stat_string[self.offset as usize..];

        let mut bytes = stat_string.bytes();

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => *to_fill = byte,
                None => {
                    ret = index + 1;
                    break;
                }
            }
        }
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
