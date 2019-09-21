use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::{boxed::Box, sync::Arc};

use fallible_collections::{boxed::FallibleBox, FallibleArc};

use core::fmt::Debug;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct FilesystemsDriver;

unsafe impl Send for FilesystemsDriver {}

#[derive(Debug, Default)]
pub struct FilesystemsOperations {
    // offset: u64,
    offset: usize,
}

impl Driver for FilesystemsDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(FilesystemsOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

// Hardcoded because no time for this bullshit.
const FILESYSTEMS: &str = "nodev procfs\n      ext2\n";

impl FileOperation for FilesystemsOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        let filesystems_string = FILESYSTEMS;
        if self.offset >= FILESYSTEMS.len() {
            return Ok(IpcResult::Done(0));
        }

        let filesystems_string = &filesystems_string[self.offset as usize..];

        let mut bytes = filesystems_string.bytes();

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
