use super::{
    Driver, FileOperation, InodeId, IpcResult, MountedFileSystem, ProcFsOperations, SysResult, VFS,
};

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

use fallible_collections::{FallibleArc, TryCollect};

use libc_binding::{Errno, OpenFlags};
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Whence};

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
        VFS.force_unlock();
        let vfs = VFS.lock();
        let mounts_bytes: Vec<u8> = vfs
            .mounted_filesystems
            .values()
            .filter_map(
                |MountedFileSystem {
                     ref source,
                     ref target,
                     ref fs_type,
                     ..
                 }| {
                    Some(
                        tryformat!(128, "{} {} {} rw 0 0\n", source, target, fs_type)
                            .ok()?
                            .into_bytes(),
                    )
                },
            )
            .flatten()
            .try_collect()?;

        Ok(Cow::from(String::from_utf8(mounts_bytes).map_err(
            |_| {
                log::error!("invalid utf8 in environ operation");
                Errno::EINVAL
            },
        )?))
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
