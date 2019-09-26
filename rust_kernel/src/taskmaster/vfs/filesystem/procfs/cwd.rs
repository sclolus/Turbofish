use super::{Driver, FileOperation, IpcResult, SysResult};
use crate::taskmaster::vfs::Path;
use crate::taskmaster::SCHEDULER;

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{Errno, Pid};

#[derive(Debug, Clone)]
pub struct CwdDriver {
    pid: Pid,
}

unsafe impl Send for CwdDriver {}

#[derive(Debug)]
pub struct CwdOperations {
    pid: Pid,
    // the cwd of the process at the moment this CwdOperations was created.
    path: Path,
    offset: usize,
}

impl Driver for CwdDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(CwdOperations::new(self.pid, 0)))?;
        Ok(IpcResult::Done(res))
    }
}

impl CwdDriver {
    pub fn new(pid: Pid) -> Self {
        Self { pid }
    }
}

impl CwdOperations {
    pub fn new(pid: Pid, offset: usize) -> Self {
        SCHEDULER.force_unlock(); //TODO: find a better way than force unlock.
        let scheduler = SCHEDULER.lock();
        let path = scheduler
            .get_thread_group(pid)
            .expect("CwdOperations::new(): The process should exist")
            .cwd
            .clone(); //TODO: unfaillible context right here
        Self { pid, path, offset }
    }
}

impl FileOperation for CwdOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        // TODO: This is dummy.
        let cwd_string = format!("{}", self.path);
        if self.offset >= cwd_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let cwd_string = &cwd_string[self.offset as usize..];

        let mut bytes = cwd_string.bytes();

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
