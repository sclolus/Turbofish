use super::SysResult;

use super::scheduler::SCHEDULER;
use super::vfs::{Path, VFS};
use libc_binding::{c_char, statfs, Errno};

use core::convert::TryFrom;

/// The statfs() system call returns information about a mounted filesystem.
/// `path` is the pathname of any file within the mounted filesystem.
/// `buf` is a  pointer  to  a  statfs structure.
pub fn sys_statfs(path: *const c_char, buf: *mut statfs) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        // Check if given pointer is not bullshit
        let (safe_path, safe_buf) = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            (v.make_checked_str(path)?, v.make_checked_ref_mut(buf)?)
        };

        let tg = scheduler.current_thread_group_mut();
        let _creds = &tg.credentials;
        let cwd = &tg.cwd;
        let path = Path::try_from(safe_path)?;

        let mut vfs = VFS.lock();
        let direntry_id = vfs.pathname_resolution(cwd, &path).or(Err(Errno::ENOENT))?;
        let inode_id = {
            vfs.dcache
                .get_entry(&direntry_id)
                .or(Err(Errno::ENOENT))?
                .inode_id
        };
        vfs.statfs(inode_id, safe_buf)?;
        Ok(0)
    })
}
