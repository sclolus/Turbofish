use super::vfs::VFS;
use super::SysResult;

use super::scheduler::SCHEDULER;
use super::vfs::Path;
use core::convert::TryFrom;
use libc_binding::c_char;

use libc_binding::stat;

pub fn sys_lstat(filename: *const c_char, buf: *mut stat) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let (safe_filename, safe_buf) = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            (
                v.make_checked_str(filename)?,
                v.make_checked_ref_mut::<stat>(buf)?,
            )
        };
        let path = Path::try_from(safe_filename)?;

        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        VFS.lock().lstat(cwd, creds, path, safe_buf)?;
        Ok(0)
    })
}
