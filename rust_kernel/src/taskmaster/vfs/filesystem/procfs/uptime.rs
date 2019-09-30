use super::{Driver, FileOperation, InodeId, IpcResult, SysResult};
use crate::drivers::pit_8253::PIT0;

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct UptimeDriver {
    inode_id: InodeId,
}

impl UptimeDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for UptimeDriver {}

impl Driver for UptimeDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(UptimeOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct UptimeOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

extern "C" {
    /// Get the pit realtime.
    fn _get_pit_time() -> u32;
}

impl FileOperation for UptimeOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        // Make sure that we are unpreemptible even if we should.
        // If PIT0 is inactive, we just assume that no seconds have elapsed yet since boot.
        // That should not happen anyway.
        let frequency = unpreemptible_context!({ PIT0.lock().period.unwrap_or(0.0) });
        let uptime = unsafe { _get_pit_time() as f32 * frequency } as usize;
        //TODO: calculate time spend in idle process.
        let _idle_process_time = 0;
        //TODO: Unfailible context.
        let uptime_string = format!(
            "{}.00 1.00",
            uptime // , idle_process_time
        );

        if self.offset >= uptime_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &uptime_string[self.offset as usize..];

        let mut bytes = version.bytes();

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => *to_fill = byte,
                None => {
                    ret = index + 1;
                    break;
                }
            }
        }
        self.offset += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}
