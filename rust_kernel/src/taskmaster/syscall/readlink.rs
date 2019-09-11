use super::SysResult;

use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use libc_binding::c_char;

use core::convert::TryFrom;

/// The readlink() function shall place the contents of the symbolic
/// link referred to by path in the buffer buf which has size
/// bufsize. If the number of bytes in the symbolic link is less than
/// bufsize, the contents of the remainder of buf are unspecified. If
/// the buf argument is not large enough to contain the link content,
/// the first bufsize bytes shall be placed in buf.
///
/// If the value of bufsize is greater than {SSIZE_MAX}, the result is
/// implementation-defined.
///
/// Upon successful completion, readlink() shall mark for update the
/// last data access timestamp of the symbolic link.
///
/// Upon successful completion, these functions shall return the count
/// of bytes placed in the buffer. Otherwise, these functions shall
/// return a value of -1, leave the buffer unchanged, and set errno to
/// indicate the error.
pub fn sys_readlink(path: *const c_char, buf: *mut c_char, bufsize: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let (safe_path, safe_buf) = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            (
                v.make_checked_str(path)?,
                v.make_checked_mut_slice(buf, bufsize as usize)?,
            )
        };

        let tg = scheduler.current_thread_group_mut();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let path = Path::try_from(safe_path)?;

        VFS.lock().readlink(cwd, creds, path, safe_buf)
    })
}
