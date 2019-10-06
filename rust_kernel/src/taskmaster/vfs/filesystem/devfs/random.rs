//! This file contains all the stuff about TTY

use super::IpcResult;
use super::SysResult;
use crate::math::random::{srand, srand_init};

use super::{Driver, FileOperation};

use super::InodeId;
use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::OpenFlags;
use sync::dead_mutex::DeadMutex;

/// This structure represents a FileOperation of type DevRandom
#[derive(Debug, Default)]
pub struct DevRandom {
    inode_id: InodeId,
}

/// Main implementation of DevRandom
impl DevRandom {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

extern "C" {
    /// Get the pit realtime.
    fn _get_pit_time() -> u32;
}

/// Main Trait implementation of DevRandom
impl FileOperation for DevRandom {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let mut last_random = 0x4242;
        for c in buf.iter_mut() {
            *c = srand::<u8>(core::u8::MAX);
            last_random = (*c as u16) << 8 | (*c as u16);
        }
        let pit_uptime = unsafe { _get_pit_time() };
        let pit_uptime = (pit_uptime >> 16) as u16 ^ (pit_uptime as u16);

        let random = (last_random ^ pit_uptime) | 1;
        srand_init(random).expect("Could not initialize /dev/random");

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
pub struct RandomDevice {
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<DevRandom>>,
}

impl RandomDevice {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        let r = Ok(Self {
            operation: Arc::try_new(DeadMutex::new(DevRandom::new(inode_id)))?,
        });

        log::info!("Random Device created !");
        r
    }
}

impl Driver for RandomDevice {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(self.operation.clone()))
    }
}
