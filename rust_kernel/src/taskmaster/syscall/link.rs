use super::scheduler::SCHEDULER;
use super::vfs::Path;
use super::vfs::VFS;
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::c_char;

/// The link() function shall create a new link (directory entry) for
/// the existing file, path1.
///
/// The path1 argument points to a pathname naming an existing
/// file. The path2 argument points to a pathname naming the new
/// directory entry to be created. The link() function shall
/// atomically create a new link for the existing file and the link
/// count of the file shall be incremented by one.
///
/// If path1 names a directory, link() shall fail unless the process
/// has appropriate privileges and the implementation supports using
/// link() on directories.
///
/// If path1 names a symbolic link, it is implementation-defined
/// whether link() follows the symbolic link, or creates a new link to
/// the symbolic link itself.
///
/// Upon successful completion, link() shall mark for update the last
/// file status change timestamp of the file. Also, the last data
/// modification and last file status change timestamps of the
/// directory that contains the new entry shall be marked for update.
///
/// If link() fails, no link shall be created and the link count of
/// the file shall remain unchanged.
///
/// The implementation may require that the calling process has
/// permission to access the existing file.
pub fn sys_link(path1: *const c_char, path2: *const c_char) -> SysResult<u32> {
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
        VFS.lock().link(cwd, creds, path1, path2)?;
        Ok(0)
    })
}
