use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::{c_char, mode_t, FileType};

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

        // this extract only permission and special bits from mode
        let pure_mode = FileType::extract_pure_mode(mode);
        let path = Path::try_from(safe_path)?;

        VFS.lock().chmod(cwd, creds, path, pure_mode)?;
        Ok(0)
    })
}
