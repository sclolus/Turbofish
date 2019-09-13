use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::{c_char, mode_t, Errno, FileType};

pub fn sys_chmod(path: *const c_char, mode: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        let safe_path = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };

        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let mode = FileType::try_from(mode)?;
        if !mode.is_pure_mode() {
            log::warn!(
                "sys_chmod({}, {:#?}) was called, mode is invalid",
                safe_path,
                mode
            );
            return Err(Errno::EINVAL);
        }
        let path = Path::try_from(safe_path)?;

        VFS.lock().chmod(cwd, creds, path, mode)?;
        Ok(0)
    })
}
