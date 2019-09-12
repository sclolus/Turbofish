use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::{c_char, gid_t, uid_t};

pub fn sys_chown(path: *const c_char, owner: uid_t, group: gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_path = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };
        let path = Path::try_from(safe_path)?;
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;

        VFS.lock().chown(cwd, creds, path, owner, group)?;
        Ok(0)
    })
}
