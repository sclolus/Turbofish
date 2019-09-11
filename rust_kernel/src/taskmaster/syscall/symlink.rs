use super::scheduler::SCHEDULER;
use super::vfs::Path;
use super::vfs::VFS;
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::c_char;

pub fn sys_symlink(path1: *const c_char, path2: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let (safe_path1, safe_path2) = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            (v.make_checked_str(path1)?, v.make_checked_str(path2)?)
        };

        let path1 = Path::try_from(safe_path1)?;
        let path2 = Path::try_from(safe_path2)?;
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        VFS.lock().symlink(cwd, creds, path1, path2)?;
        Ok(0)
    })
}
