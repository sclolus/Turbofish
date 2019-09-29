//! this is the frame buffer device
use super::IpcResult;
use super::SysResult;
use screen::AdvancedGraphic;
use terminal::SCREEN_MONAD;

use super::{Driver, FileOperation};

use super::InodeId;
use alloc::sync::Arc;
use core::cmp;
use core::slice;
use fallible_collections::FallibleArc;
use libc_binding::OpenFlags;
use sync::dead_mutex::DeadMutex;

/// This structure represents a FileOperation of type DevFb
#[derive(Debug, Default)]
pub struct DevFb {
    inode_id: InodeId,
}

/// Main implementation of DevFb
impl DevFb {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

/// Main Trait implementation of DevFb
impl FileOperation for DevFb {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        for x in buf.iter_mut() {
            *x = 0;
        }
        return Ok(IpcResult::Done(buf.len() as u32));
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        let mut screen = SCREEN_MONAD.lock();
        screen
            .draw_graphic_buffer(|ptr, width, height, bpp| {
                let s = unsafe {
                    slice::from_raw_parts_mut(ptr, cmp::min(width * height * bpp / 8, buf.len()))
                };
                // s.write(buf);
                s.copy_from_slice(buf);
                Ok(())
            })
            .expect("draw graphic buffer failed");
        let r = screen.query_graphic_infos();
        let data_writen = match r {
            Ok((height, width, bpp)) => cmp::min(width * height * bpp / 8, buf.len()),
            Err(e) => {
                log::error!("{:?}", e);
                0
            }
        };
        Ok(IpcResult::Done(data_writen as u32))
    }
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }
}

#[derive(Debug)]
pub struct FbDevice {
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<DevFb>>,
}

impl FbDevice {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        let r = Ok(Self {
            operation: Arc::try_new(DeadMutex::new(DevFb::new(inode_id)))?,
        });
        log::info!("Fb Device created !");
        r
    }
}

impl Driver for FbDevice {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(self.operation.clone()))
    }
}
