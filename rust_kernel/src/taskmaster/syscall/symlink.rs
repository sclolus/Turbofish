use super::scheduler::SCHEDULER;
use super::vfs::Path;
use super::vfs::VFS;
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::c_char;

pub fn sys_symlink(target: *const c_char, linkname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let (safe_target, safe_linkname) = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            (v.make_checked_str(target)?, v.make_checked_str(linkname)?)
        };

        let linkname = Path::try_from(safe_linkname)?;
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        VFS.lock().symlink(cwd, creds, safe_target, linkname)?;
        Ok(0)
    })
}
