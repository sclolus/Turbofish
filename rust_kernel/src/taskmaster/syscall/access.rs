use super::scheduler::SCHEDULER;
use super::statfn;
use super::vfs::{Path, VFS};
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::{c_char, stat, Amode, Errno, FileType};

pub fn sys_access(path: *const c_char, amode: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_path = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };
        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;

        let path = Path::try_from(safe_path)?;
        let mut buf: stat = Default::default();
        statfn(&scheduler, path, &mut buf)?;
        let amode = Amode::from_bits(amode).ok_or(Errno::EINVAL)?;
        let filetype =
            FileType::from_bits(buf.st_mode as u16).expect("filetypes bits should be valid");
        let has_access = if creds.uid == 0 {
            // If a process has appropriate privileges:
            //
            //  If read, write, or directory search permission is
            //  requested, access shall be granted.
            //
            //  If execute permission is requested, access shall be
            //  granted if execute permission is granted to at least one
            //  user by the file permission bits or by an alternate access
            //  control mechanism; otherwise, access shall be denied.
            if !filetype.is_directory() && amode.contains(Amode::X_OK) {
                filetype.contains(FileType::OTHER_EXECUTE_PERMISSION)
                    || filetype.contains(FileType::GROUP_EXECUTE_PERMISSION)
                    || filetype.contains(FileType::USER_EXECUTE_PERMISSION)
            } else {
                true
            }
        } else {
            // Otherwise:
            //  The file permission bits of a file contain read, write,
            //  and execute/search permissions for the file owner class,
            //  file group class, and file other class.

            //  Access shall be granted if an alternate access control
            //  mechanism is not enabled and the requested access
            //  permission bit is set for the class (file owner class,
            //  file group class, or file other class) to which the
            //  process belongs, or if an alternate access control
            //  mechanism is enabled and it allows the requested access;
            //  otherwise, access shall be denied.
            let is_file_owner = buf.st_uid == creds.uid;
            let is_group_owner = buf.st_gid == creds.gid;

            if is_file_owner {
                filetype.owner_access() == amode
            } else if is_group_owner {
                filetype.group_access() == amode
            } else {
                filetype.other_access() == amode
            }
        };
        if has_access {
            Ok(0)
        } else {
            Err(Errno::EACCES)
        }
    })
}
