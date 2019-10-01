use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};
use crate::drivers::pit_8253::PIT0;

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::OpenFlags;
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Errno, Whence};

#[derive(Debug, Clone)]
pub struct ProcStatDriver {
    inode_id: InodeId,
}

impl ProcStatDriver {
    pub fn new(inode_id: InodeId) -> Self {
        Self { inode_id }
    }
}

unsafe impl Send for ProcStatDriver {}

impl Driver for ProcStatDriver {
    fn open(&mut self, _flags: OpenFlags) -> SysResult<IpcResult<Arc<Mutex<dyn FileOperation>>>> {
        let res = Arc::try_new(Mutex::new(ProcStatOperations {
            inode_id: self.inode_id,
            offset: 0,
        }))?;
        Ok(IpcResult::Done(res))
    }
}

#[derive(Debug, Default)]
pub struct ProcStatOperations {
    inode_id: InodeId,
    offset: usize,
}

const PROC_STAT_HARDCODE: &'static str = "cpu 42 42 42 42 42 42 42 42 42 42\n\
                                          cpu0 42 42 42 42 42 42 42 42 42 42\n\
                                          page 42 42\n\
                                          swap 1 42\n\
                                          intr 42\n\
                                          ctx 42\n\
                                          btime 1 1\n\
                                          processes 42\n\
                                          procs_running 42\n\
                                          procs_blocked 42\n\
                                          softirq 42 42 42 42 42 42 42 42 42 42\n";

extern "C" {
    /// Get the pit realtime.
    fn _get_pit_time() -> u32;
}

impl ProcFsOperations for ProcStatOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        let frequency = unpreemptible_context!({ PIT0.lock().period.unwrap_or(0.0) });

        let uptime = unsafe { (dbg!(_get_pit_time()) as f32 * frequency) * 100.0 } as usize; // TODO: USER_HZ
                                                                                             // This is dummy.
                                                                                             // eprintln!("f: {}, uptime: {}", frequency, uptime);
        let proc_stat_string = tryformat!(
            4096,
            "cpu {} {} {} {} {} {} {} {} {} {}\n\
             cpu0 {} {} {} {} {} {} {} {} {} {}\n\
             page 42 42\n\
             swap 1 42\n\
             intr 42\n\
             ctx 42\n\
             btime {}\n\
             processes 42\n\
             procs_running 42\n\
             procs_blocked 42\n\
             softirq 42 42 42 42 42 42 42 42 42 42\n",
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
            uptime,
        )?;

        Ok(Cow::from(proc_stat_string))
    }
}

impl FileOperation for ProcStatOperations {
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

impl Drop for ProcStatOperations {
    fn drop(&mut self) {
        VFS.lock().close_file_operation(self.inode_id);
    }
}
