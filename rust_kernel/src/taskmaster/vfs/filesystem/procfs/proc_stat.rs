use super::{Driver, FileOperation, InodeId, IpcResult, ProcFsOperations, SysResult, VFS};
use crate::drivers::pit_8253::PIT0;
use crate::taskmaster::global_time::GLOBAL_TIME;

use alloc::borrow::Cow;
use alloc::sync::Arc;

use fallible_collections::FallibleArc;

use libc_binding::{OpenFlags, HZ};
use sync::DeadMutex;

type Mutex<T> = DeadMutex<T>;

use libc_binding::{off_t, Whence};

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

extern "C" {
    /// Get the pit realtime.
    fn _get_pit_time() -> u32;
}

impl ProcFsOperations for ProcStatOperations {
    fn get_offset(&mut self) -> &mut usize {
        &mut self.offset
    }

    fn get_seq_string(&self) -> SysResult<Cow<str>> {
        let global_time = unsafe { GLOBAL_TIME.as_ref().unwrap() };
        let frequency = global_time.cpu_frequency();

        let pit_frequency = unpreemptible_context!({ PIT0.lock().period.unwrap_or(0.0) });

        let uptime = unsafe { (_get_pit_time() as f32 * pit_frequency) * 100.0 } as usize; // TODO: USER_HZ

        let hertz = HZ as u64;
        let user = global_time.global_user_time().as_secs() * hertz;
        let nice = 0 * hertz;
        let system = global_time.global_system_time().as_secs() * hertz;
        let idle = global_time.global_idle_time().as_secs() * hertz;

        let boiler = 42;

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
            user,
            nice,
            system,
            idle,
            boiler,
            boiler,
            boiler,
            boiler,
            boiler,
            boiler,
            user,
            nice,
            system,
            idle,
            boiler,
            boiler,
            boiler,
            boiler,
            boiler,
            boiler,
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
