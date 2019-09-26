use super::{Driver, FileOperation, IpcResult, SysResult};
// use crate::taskmaster::vfs::VFS;

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct MountsDriver;

unsafe impl Send for MountsDriver {}

#[derive(Debug, Default)]
pub struct MountsOperations {
    // offset: u64,
    offset: usize,
}

impl Driver for MountsDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(MountsOperations { offset: 0 }))?;
        Ok(IpcResult::Done(res))
    }
}

impl FileOperation for MountsOperations {
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        // let vfs = VFS.force_unlock();

        // vfs.mounted_filesystems.iter().map(|fs| fs.lock())

        let hardcoded_mounts_string = "/dev/sda1 / ext2 rw 0 0\n\
                                       proc /proc procfs ro 0 0\n";
        let mounts_string = hardcoded_mounts_string;
        if self.offset >= mounts_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let mounts_string = &mounts_string[self.offset as usize..];

        let mut bytes = mounts_string.bytes();

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
