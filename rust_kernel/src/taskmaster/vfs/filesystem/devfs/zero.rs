//! This file contains all the stuff about TTY

use super::IpcResult;
use super::SysResult;

use super::{Driver, FileOperation};

use super::InodeId;
use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::OpenFlags;
use sync::dead_mutex::DeadMutex;

/// This structure represents a FileOperation of type DevZero
#[derive(Debug, Default)]
pub struct DevZero {
    inode_id: InodeId,
}

/// Main implementation of DevZero
impl DevZero {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

/// Main Trait implementation of DevZero
impl FileOperation for DevZero {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        for x in buf.iter_mut() {
            *x = 0;
        }
        return Ok(IpcResult::Done(buf.len() as u32));
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Ok(IpcResult::Done(buf.len() as _))
    }
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }
}

#[derive(Debug)]
pub struct ZeroDevice {
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<DevZero>>,
}

impl ZeroDevice {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        let r = Ok(Self {
            operation: Arc::try_new(DeadMutex::new(DevZero::new(inode_id)))?,
        });
        log::info!("Zero Device created !");
        r
    }
}

impl Driver for ZeroDevice {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(self.operation.clone()))
    }
}
