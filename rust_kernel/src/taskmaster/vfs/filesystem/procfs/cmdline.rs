use super::{Driver, FileOperation, InodeId, IpcResult, SysResult, VFS};
use crate::taskmaster::SCHEDULER;

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{Errno, Pid};

#[derive(Debug, Clone)]
pub struct CmdlineDriver {
    inode_id: InodeId,
    pid: Pid,
}

unsafe impl Send for CmdlineDriver {}

#[derive(Debug)]
pub struct CmdlineOperations {
    inode_id: InodeId,
    pid: Pid,
    // the cwd of the process at the moment this CmdlineOperations was created.
    offset: usize,
}

impl Driver for CmdlineDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(CmdlineOperations::new(
            self.inode_id,
            self.pid,
            0,
        )))?;
        Ok(IpcResult::Done(res))
    }
}

impl CmdlineDriver {
    pub fn new(inode_id: InodeId, pid: Pid) -> Self {
        Self { inode_id, pid }
    }
}

impl CmdlineOperations {
    pub fn new(inode_id: InodeId, pid: Pid, offset: usize) -> Self {
        Self {
            inode_id,
            pid,
            offset,
        }
    }
}

impl FileOperation for CmdlineOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let cmdline = {
            match scheduler
                .get_thread_group(self.pid)
                .expect("CmdlineOperations::read(): The Process should exist")
                .argv
                .as_ref()
            {
                Some(cmdline) => cmdline,
                None => return Ok(IpcResult::Done(0)),
            }
        };

        let mut bytes = cmdline
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

impl Drop for CmdlineOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
