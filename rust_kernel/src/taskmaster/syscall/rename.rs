use super::scheduler::SCHEDULER;
use super::vfs::Path;
use super::vfs::VFS;
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::c_char;

/// The rename() function shall change the name of a file. The old
/// argument points to the pathname of the file to be renamed. The new
/// argument points to the new pathname of the file. [CX] [Option
/// Start] If the new argument does not resolve to an existing
/// directory entry for a file of type directory and the new argument
/// contains at least one non- <slash> character and ends with one or
/// more trailing <slash> characters after all symbolic links have
/// been processed, rename() shall fail.
///
/// If either the old or new argument names a symbolic link, rename()
/// shall operate on the symbolic link itself, and shall not resolve
/// the last component of the argument. If the old argument and the
/// new argument resolve to either the same existing directory entry
/// or different directory entries for the same existing file,
/// rename() shall return successfully and perform no other action.
///
/// If the old argument points to the pathname of a file that is not a
/// directory, the new argument shall not point to the pathname of a
/// directory. If the link named by the new argument exists, it shall
/// be removed and old renamed to new. In this case, a link named new
/// shall remain visible to other threads throughout the renaming
/// operation and refer either to the file referred to by new or old
/// before the operation began. Write access permission is required
/// for both the directory containing old and the directory containing
/// new.
///
/// If the old argument points to the pathname of a directory, the new
/// argument shall not point to the pathname of a file that is not a
/// directory. If the directory named by the new argument exists, it
/// shall be removed and old renamed to new. In this case, a link
/// named new shall exist throughout the renaming operation and shall
/// refer either to the directory referred to by new or old before the
/// operation began. If new names an existing directory, it shall be
/// required to be an empty directory.
///
/// If either pathname argument refers to a path whose final component
/// is either dot or dot-dot, rename() shall fail.
///
/// If the old argument points to a pathname of a symbolic link, the
/// symbolic link shall be renamed. If the new argument points to a
/// pathname of a symbolic link, the symbolic link shall be removed.
///
/// The old pathname shall not name an ancestor directory of the new
/// pathname. Write access permission is required for the directory
/// containing old and the directory containing new. If the old
/// argument points to the pathname of a directory, write access
/// permission may be required for the directory named by old, and, if
/// it exists, the directory named by new.
///
/// If the link named by the new argument exists and the file's link
/// count becomes 0 when it is removed and no process has the file
/// open, the space occupied by the file shall be freed and the file
/// shall no longer be accessible. If one or more processes have the
/// file open when the last link is removed, the link shall be removed
/// before rename() returns, but the removal of the file contents
/// shall be postponed until all references to the file are closed.
///
/// Upon successful completion, rename() shall mark for update the
/// last data modification and last file status change timestamps of
/// the parent directory of each file.
///
/// If the rename() function fails for any reason other than [EIO],
/// any file named by new shall be unaffected.
pub fn sys_rename(old: *const c_char, new: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let (safe_old, safe_new) = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            (v.make_checked_str(old)?, v.make_checked_str(new)?)
        };
        let old = Path::try_from(safe_old)?;
        let new = Path::try_from(safe_new)?;
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        VFS.lock().rename(cwd, creds, old, new)?;
        Ok(0)
    })
}
