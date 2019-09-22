use super::{Driver, FileOperation, IpcResult, SysResult};
use crate::taskmaster::vfs::Path;
use crate::taskmaster::SCHEDULER;

use alloc::{boxed::Box, sync::Arc};

use fallible_collections::{boxed::FallibleBox, FallibleArc};

use core::fmt::Debug;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{Errno, Pid};

#[derive(Debug, Clone)]
pub struct EnvironDriver {
    pid: Pid,
}

unsafe impl Send for EnvironDriver {}

#[derive(Debug)]
pub struct EnvironOperations {
    pid: Pid,
    // the cwd of the process at the moment this EnvironOperations was created.
    offset: usize,
}

impl Driver for EnvironDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(EnvironOperations::new(self.pid, 0)))?;
        Ok(IpcResult::Done(res))
    }
}

impl EnvironDriver {
    pub fn new(pid: Pid) -> Self {
        Self { pid }
    }
}

impl EnvironOperations {
    pub fn new(pid: Pid, offset: usize) -> Self {
        Self { pid, offset }
    }
}

impl FileOperation for EnvironOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let environ = {
            match scheduler
                .get_thread_group(self.pid)
                .ok_or(Errno::ESRCH)?
                // .expect("EnvironOperations::read(): The Process should exist")
                .environ
                .as_ref()
            {
                Some(environ) => environ,
                None => return Ok(IpcResult::Done(0)),
            }
        };

        let mut bytes = environ
            .strings()
            .flat_map(|s| s.iter().map(|b| *b as u8))
            .skip(self.offset);

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => {
                    ret = index + 1;
                    *to_fill = byte
                }
                None => {
                    break;
                }
            }
        }
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
