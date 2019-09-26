use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct ProcStatDriver;

unsafe impl Send for ProcStatDriver {}

impl Driver for ProcStatDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(ProcStatOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct ProcStatOperations {
    // offset: u64,
    offset: usize,
}

const PROC_STAT_HARDCODE: &'static str = "cpu 0 0 0 0 0 0 0 0 0 0\n\
                                          cpu0 0 0 0 0 0 0 0 0 0 0\n\
                                          page 0 0\n\
                                          swap 1 0\n\
                                          intr 0\n\
                                          ctx 0\n\
                                          btime\n\
                                          processes\n\
                                          procs_running 1\n\
                                          procs_blocked 2\n\
                                          softirq 0 0 0 0 0 0 0 0 0 0\n";

impl FileOperation for ProcStatOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        if self.offset >= PROC_STAT_HARDCODE.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &PROC_STAT_HARDCODE[self.offset as usize..];

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
