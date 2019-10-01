use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct VersionDriver {
    inode_id: InodeId,
}

impl VersionDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for VersionDriver {}

impl Driver for VersionDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(VersionOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct VersionOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

const KERNEL_VERSION: &'static str = "Turbofish v?.?.?\n";

impl FileOperation for VersionOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.seq_read(buf)
    }
}

impl ProcFsOperations for VersionOperations {
    fn get_seq_string(&self) -> SysResult<&str> {
        Ok(KERNEL_VERSION)
    }
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
}

impl Drop for VersionOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
