use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;
use libc_binding::{c_char, mode_t, Errno, FileType};

pub fn sys_mkdir(path: *const c_char, mut mode: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_path = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };
        let tg = scheduler.current_thread_group_mut();

        let mask = tg.umask;
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let path = Path::try_from(safe_path)?;
        // Mask out the bits of mode which are set in umask.
        mode = mode & !mask;
        let mode = FileType::from_bits(mode as u16).ok_or(Errno::EINVAL)?;
        VFS.lock().mkdir(cwd, creds, path, mode)?;
        Ok(0)
    })
}
