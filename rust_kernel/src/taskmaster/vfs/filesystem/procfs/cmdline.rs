use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};
use crate::taskmaster::SCHEDULER;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use fallible_collections::{FallibleArc, TryCollect};

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Whence};
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

impl ProcFsOperations for CmdlineOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn get_seq_string(&self) -> SysResult<Cow<str>> {
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
                None => return Ok(Cow::from("")),
            }
        };

        let mut bytes: Vec<u8> = cmdline
            .strings()
            .flat_map(|s| s.iter().map(|b| *b as u8))
            .skip(self.offset)
            .try_collect()?;

        Ok(Cow::from(String::from_utf8(bytes).map_err(|_| {
            log::error!("invalid utf8 in environ operation");
            Errno::EINVAL
        })?))
    }
}

impl FileOperation for CmdlineOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.seq_read(buf)
    }

    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        self.proc_lseek(offset, whence)
    }
}

impl Drop for CmdlineOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
