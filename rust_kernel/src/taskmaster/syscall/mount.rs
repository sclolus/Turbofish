use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;
use core::ffi::c_void;

use libc_binding::c_char;

/// Simply mount source on directory target for the moment
pub fn sys_mount(
    source: *const c_char,
    target: *const c_char,
    _filesystemtype: *const c_char,
    _mountflags: u32,
    _data: *const c_void,
) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        let (safe_source, safe_target) = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            (v.make_checked_str(source)?, v.make_checked_str(target)?)
        };

        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let source = Path::try_from(safe_source)?;
        let target = Path::try_from(safe_target)?;

        VFS.lock().mount(cwd, creds, source, target)?;
        Ok(0)
    })
}
