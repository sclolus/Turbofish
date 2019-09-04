use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;

use core::convert::TryFrom;
use core::convert::TryInto;
use libc_binding::{dirent, DIR};

use crate::memory::tools::AllocFlags;

/// The chdir() function shall cause the directory named by the
/// pathname pointed to by the path argument to become the current
/// working directory; that is, the starting point for path searches
/// for pathnames not beginning with '/'.
pub fn sys_chdir(buf: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

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

        let mut current = super::vfs::Current {
            cwd: super::vfs::DirectoryEntryId::new(2),
            uid: 0,
            euid: 0,
            gid: 0,
            egid: 0,
        };
        let path = super::vfs::Path::try_from(safe_buf)?;
        //TODO: get pathname resolution from vfs + stock path in thread_group
        Ok(0)
    })
}
