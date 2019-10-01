use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};

use alloc::sync::Arc;

use alloc::borrow::Cow;
use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Errno, Whence};

#[derive(Debug, Clone)]
pub struct MountsDriver {
    inode_id: InodeId,
}

impl MountsDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for MountsDriver {}

#[derive(Debug, Default)]
pub struct MountsOperations {
    inode_id: InodeId,
    offset: usize,
}

impl Driver for MountsDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(MountsOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

impl FileOperation for MountsOperations {
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

impl ProcFsOperations for MountsOperations {
    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        let hardcoded_mounts_string = "/dev/sda1 / ext2 rw 0 0\n\
                                       proc /proc procfs ro 0 0\n";
        Ok(Cow::from(hardcoded_mounts_string))
    }
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }
}

impl Drop for MountsOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
