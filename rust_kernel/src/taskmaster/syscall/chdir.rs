use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};

use core::convert::TryFrom;
use core::convert::TryInto;

/// The chdir() function shall cause the directory named by the
/// pathname pointed to by the path argument to become the current
/// working directory; that is, the starting point for path searches
/// for pathnames not beginning with '/'.
pub fn sys_chdir(buf: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            let c_string: CString = (&v, buf).try_into()?;

            unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    buf as *const u8,
                    c_string.len(),
                ))
            }
        };

        let tg = scheduler.current_thread_group_mut();
        // let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let path = Path::try_from(safe_buf)?;

        let mut vfs = VFS.lock();
        let direntry_id = vfs.pathname_resolution(cwd, path)?;

        let posix_path = vfs.dentry_path(direntry_id)?;
        assert!(posix_path.is_absolute());

        tg.cwd = posix_path;

        Ok(0)
    })
}
