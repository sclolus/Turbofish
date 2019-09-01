use super::{Driver, FileOperation, IpcResult, SysResult};
use alloc::sync::Arc;
use ext2::Ext2Filesystem;
use libc_binding::{off_t, Whence};
use sync::DeadMutex;

/// a driver of an ext2 file
#[derive(Debug)]
pub struct Ext2DriverFile {
    ext2: Arc<DeadMutex<Ext2Filesystem>>,
    inode_nbr: u32,
}

impl Ext2DriverFile {
    pub fn new(ext2: Arc<DeadMutex<Ext2Filesystem>>, inode_nbr: u32) -> Self {
        Self { ext2, inode_nbr }
    }
}

impl Driver for Ext2DriverFile {
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(Arc::new(DeadMutex::new(
            Ext2FileOperation::new(self.ext2.clone(), self.inode_nbr),
        ))))
    }
}

/// a file operation of an ext2 file
#[derive(Debug)]
pub struct Ext2FileOperation {
    ext2_driver_file: Arc<DeadMutex<Ext2Filesystem>>,
    inode_nbr: u32,
    offset: u32,
}

impl Ext2FileOperation {
    fn new(ext2_driver_file: Arc<DeadMutex<Ext2Filesystem>>, inode_nbr: u32) -> Self {
        Self {
            ext2_driver_file,
            inode_nbr,
            offset: 0,
        }
    }
}

impl FileOperation for Ext2FileOperation {
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }

    fn lseek(&mut self, _offset: off_t, _whence: Whence) -> SysResult<off_t> {
        unimplemented!();
    }
}
