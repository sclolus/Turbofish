use super::{Driver, FileOperation, InodeId, IpcResult, SysResult, VFS};

use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;

#[derive(Debug, Clone)]
pub struct LoadavgDriver {
    inode_id: InodeId,
}

impl LoadavgDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for LoadavgDriver {}

impl Driver for LoadavgDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(LoadavgOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct LoadavgOperations {
    // offset: u64,
    inode_id: InodeId,
    offset: usize,
}

impl FileOperation for LoadavgOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }

        //TODO: Unfailible context.
        //TODO: This is dummy.
        let load_avg_string = format!("0.00 0.00 0.00 0/0 1");

        if self.offset >= load_avg_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let version = &load_avg_string[self.offset as usize..];

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

impl Drop for LoadavgOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
