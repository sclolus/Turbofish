//! This file contains all the stuff about Fifo

use super::SysResult;

use super::get_file_op_uid;
use super::{Driver, FileOperation, IpcResult};

use super::vfs::InodeId;

use alloc::sync::Arc;
use sync::DeadMutex;

use fallible_collections::arc::FallibleArc;
use libc_binding::{Errno, OpenFlags};

/// Stucture of FifoDriver
#[derive(Debug)]
pub struct FifoDriver {
    inode_id: InodeId,
    operation: Arc<DeadMutex<FifoFileOperation>>,
    readers: usize,
    writers: usize,
}

/// Main implementation of FifoDriver
impl FifoDriver {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        log::info!("Fifo Driver created !");
        Ok(Self {
            inode_id,
            operation: Arc::try_new(DeadMutex::new(FifoFileOperation::new(inode_id)))?,
            readers: 0,
            writers: 0,
        })
    }
}

/// Driver trait implementation of FifoDriver
impl Driver for FifoDriver {
    fn open(
        &mut self,
        flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        log::info!("Opening fifo");

        if flags.contains(OpenFlags::O_RDONLY) {
            if self.writers == 0 {
                Ok(IpcResult::Wait(
                    self.operation.clone(),
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.operation.clone()))
            }
        } else if flags.contains(OpenFlags::O_WRONLY) {
            if self.readers == 0 {
                Ok(IpcResult::Wait(
                    self.operation.clone(),
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.operation.clone()))
            }
        } else {
            Err(Errno::EINVAL)
        }
    }
}

/// Drop boilerplate
impl Drop for FifoDriver {
    fn drop(&mut self) {
        log::info!("FIFO droped !",);
    }
}

/// This structure represents a FileOperation of type Fifo
#[derive(Debug, Default)]
pub struct FifoFileOperation {
    file_op_uid: usize,
    inode_id: InodeId,
}

/// Main implementation for Fifo file operation
impl FifoFileOperation {
    pub fn new(inode_id: InodeId) -> Self {
        log::info!("New fifo file operation registered");
        Self {
            file_op_uid: get_file_op_uid(),
            inode_id,
        }
    }
}

/// Main Trait implementation
impl FileOperation for FifoFileOperation {
    fn register(&mut self, _flags: OpenFlags) {
        unimplemented!();
    }
    fn unregister(&mut self, _flags: OpenFlags) {
        unimplemented!();
    }
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
}

/// Some boilerplate to check if all is okay
impl Drop for FifoFileOperation {
    fn drop(&mut self) {
        println!("Fifo droped !");
    }
}
