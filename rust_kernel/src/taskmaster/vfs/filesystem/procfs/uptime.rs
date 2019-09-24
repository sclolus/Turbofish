use super::{Driver, FileOperation, IpcResult, SysResult};
use crate::drivers::pit_8253::PIT0;

use alloc::{boxed::Box, sync::Arc};

use fallible_collections::{boxed::FallibleBox, FallibleArc};

use core::fmt::Debug;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct UptimeDriver;

unsafe impl Send for UptimeDriver {}

impl Driver for UptimeDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(UptimeOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct UptimeOperations {
    // offset: u64,
    offset: usize,
}

extern "C" {
    /// Get the pit realtime.
    fn _get_pit_time() -> u32;
}

impl FileOperation for UptimeOperations {
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
        let idle_process_time = 0;
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
