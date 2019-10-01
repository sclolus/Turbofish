use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Errno, Whence};

#[derive(Debug, Clone)]
pub struct LoadavgDriver {
    inode_id: InodeId,
}

impl LoadavgDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for LoadavgDriver {}

impl Driver for LoadavgDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(LoadavgOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct LoadavgOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

impl ProcFsOperations for LoadavgOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        let load_avg_string = format!("0.00 0.00 0.00 0/0 1");
        Ok(Cow::from(load_avg_string))
    }
}

impl FileOperation for LoadavgOperations {
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

impl Drop for LoadavgOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
