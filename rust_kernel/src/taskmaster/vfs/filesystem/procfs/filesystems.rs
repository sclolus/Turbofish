use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::sync::Arc;

use alloc::borrow::Cow;
use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Whence};

#[derive(Debug, Clone)]
pub struct FilesystemsDriver {
    inode_id: InodeId,
}

impl FilesystemsDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for FilesystemsDriver {}

#[derive(Debug, Default)]
pub struct FilesystemsOperations {
    inode_id: InodeId,
    offset: usize,
}

impl Driver for FilesystemsDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(FilesystemsOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

// Hardcoded because no time for this bullshit.
const FILESYSTEMS: &str = "nodev procfs\n      ext2\n";

impl FileOperation for FilesystemsOperations {
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

impl ProcFsOperations for FilesystemsOperations {
    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        Ok(Cow::from(FILESYSTEMS))
    }
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
}

impl Drop for FilesystemsOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
