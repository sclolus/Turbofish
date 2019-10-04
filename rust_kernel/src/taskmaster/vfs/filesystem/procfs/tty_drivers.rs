use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};
// use crate::taskmaster::scheduler::Credentials;
// use crate::taskmaster::SCHEDULER;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::sync::Arc;
// use alloc::vec::Vec;

use fallible_collections::{FallibleArc, TryCollect};

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::Errno;
use libc_binding::{off_t, Whence};

#[derive(Debug, Clone)]
pub struct TtyDriversDriver {
    inode_id: InodeId,
}

unsafe impl Send for TtyDriversDriver {}

#[derive(Debug)]
pub struct TtyDriversOperations {
    inode_id: InodeId,
    // the cwd of the process at the moment this TtyDriversOperations was created.
    offset: usize,
}

impl Driver for TtyDriversDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(TtyDriversOperations::new(self.inode_id, 0)))?;
        Ok(IpcResult::Done(res))
    }
}

impl TtyDriversDriver {
    #[allow(unused)]
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

impl TtyDriversOperations {
    pub fn new(inode_id: InodeId, offset: usize) -> Self {
        Self { inode_id, offset }
    }
}

impl ProcFsOperations for TtyDriversOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        // VFS.force_unlock();
        // let vfs = VFS.lock();
        // let root_dir = Filename::from_str_unwrap("/");
        // let devfs_dir = Filename::from_str_unwrap("/dev");
        // let creds = Credentials::ROOT;

        // let dev_entries = vfs.opendir(&root_dir, &creds, &devfs_dir)?;

        // let tty_drivers = {
        //     match vfs
        //         .get_thread_group(self.pid)
        //         .expect("TtyDriversOperations::read(): The Process should exist")
        //         .argv
        //         .as_ref()
        //     {
        //         Some(tty_drivers) => tty_drivers,
        //         None => return Ok(Cow::from("")),
        //     }
        // };

        // let mut bytes: Vec<u8> = tty_drivers
        //     .strings()
        //     .flat_map(|s| s.iter().map(|b| *b as u8))
        //     .skip(self.offset)
        //     .try_collect()?;
        let bytes = "dummy".bytes().try_collect()?;

        Ok(Cow::from(String::from_utf8(bytes).map_err(|_| {
            log::error!("invalid utf8 in environ operation");
            Errno::EINVAL
        })?))
    }
}

impl FileOperation for TtyDriversOperations {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.seq_read(buf)
    }

    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        self.proc_lseek(offset, whence)
    }
}

impl Drop for TtyDriversOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
