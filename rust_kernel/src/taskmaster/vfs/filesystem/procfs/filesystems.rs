use super::{Driver, FileOperation, InodeId, IpcResult, SysResult};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct FilesystemsDriver {
    inode_id: InodeId,
}

impl FilesystemsDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for FilesystemsDriver {}

#[derive(Debug, Default)]
pub struct FilesystemsOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

impl Driver for FilesystemsDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(FilesystemsOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

// Hardcoded because no time for this bullshit.
const FILESYSTEMS: &str = "nodev procfs\n      ext2\n";

impl FileOperation for FilesystemsOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

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
