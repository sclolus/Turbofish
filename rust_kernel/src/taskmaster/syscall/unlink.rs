//! Delete a name and possibly the file it refers to
use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;
use libc_binding::c_char;

pub fn sys_unlink(path: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_path = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;

        let path = Path::try_from(safe_path)?;
        VFS.lock().unlink(cwd, creds, path)?;
        Ok(0)
    })
}
