//! This file contains all the stuff about TTY

use super::IpcResult;
use super::SysResult;

use super::{Driver, FileOperation};

use super::InodeId;
use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::OpenFlags;
use sync::dead_mutex::DeadMutex;

/// This structure represents a FileOperation of type DevNull
#[derive(Debug, Default)]
pub struct DevNull {
    inode_id: InodeId,
}

/// Main implementation of DevNull
impl DevNull {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

/// Main Trait implementation of DevNull
impl FileOperation for DevNull {
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        return Ok(IpcResult::Done(0));
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Ok(IpcResult::Done(buf.len() as _))
    }
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }
}

#[derive(Debug)]
pub struct NullDevice {
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<DevNull>>,
}

impl NullDevice {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        let r = Ok(Self {
            operation: Arc::try_new(DeadMutex::new(DevNull::new(inode_id)))?,
        });
        log::info!("Null Device created !");
        r
    }
}

impl Driver for NullDevice {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(self.operation.clone()))
    }
}
