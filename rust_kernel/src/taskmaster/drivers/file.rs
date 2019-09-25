use super::{Credentials, Driver, FileOperation, IpcResult, SysResult};
use super::{InodeId, VFS};
use alloc::sync::Arc;
use libc_binding::{gid_t, off_t, statfs, uid_t, Errno, FileType, OpenFlags, Whence};
use sync::DeadMutex;

/// a driver of an ext2 file
#[derive(Debug)]
pub struct Ext2DriverFile {
    inode_id: InodeId,
}

impl Ext2DriverFile {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

impl Driver for Ext2DriverFile {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(Arc::new(DeadMutex::new(
            Ext2FileOperation::new(self.inode_id),
        ))))
    }
}

/// a file operation of an ext2 file
#[derive(Debug)]
pub struct Ext2FileOperation {
    inode_id: InodeId,
    offset: u64,
}

impl Ext2FileOperation {
    fn new(inode_id: InodeId) -> Self {
        Self {
            inode_id,
            offset: 0,
        }
    }
}

impl FileOperation for Ext2FileOperation {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let res = VFS
            .lock()
            .get_inode(self.inode_id)
            .expect("no such inode")
            .read(&mut self.offset, buf)? as u32;
        Ok(IpcResult::Done(res))
    }

    fn fstatfs(&mut self, buf: &mut statfs) -> SysResult<u32> {
        VFS.lock().fstatfs(self.inode_id, buf)?;
        Ok(0)
    }

    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        let res = VFS
            .lock()
            .get_inode(self.inode_id)
            .expect("no such inode")
            .write(&mut self.offset, buf)? as u32;
        Ok(IpcResult::Done(res))
    }

    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        if offset == core::i64::MIN {
            // volontary trash i64 min value to avoid -offset ==
            // offset
            return Err(Errno::EINVAL);
        }
        let new_offset = match whence {
            Whence::SeekCur => {
                if offset < 0 {
                    self.offset
                        .checked_sub((-offset) as u64)
                        .ok_or(Errno::EINVAL)?
                } else {
                    self.offset
                        .checked_add(offset as u64)
                        .ok_or(Errno::EINVAL)?
                }
            }
            Whence::SeekSet => {
                if offset < 0 {
                    return Err(Errno::EINVAL);
                }
                offset as u64
            }
            Whence::SeekEnd => unimplemented!(),
        };
        // if new_offset > self.partition_size {
        //     return Err(Errno::EINVAL);
        // }
        self.offset = new_offset;
        Ok(self.offset as off_t)
    }

    fn fchmod(&mut self, creds: &Credentials, mode: FileType) -> SysResult<u32> {
        VFS.lock().fchmod(creds, self.inode_id, mode)?;
        Ok(0)
    }

    fn fchown(&mut self, creds: &Credentials, owner: uid_t, group: gid_t) -> SysResult<u32> {
        VFS.lock().fchown(creds, self.inode_id, owner, group)?;
        Ok(0)
    }
}

impl Drop for Ext2FileOperation {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
