use super::SysResult;
use super::SCHEDULER;

use libc_binding::{mode_t, FileType};

/// The umask() function shall set the file mode creation mask of
/// the process to cmask and return the previous value of the mask.
/// Only the file permission bits of cmask (see <sys/stat.h>) are used;
/// the meaning of the other bits is implementation-defined.
///
/// The file mode creation mask of the process is used to turn off
/// permission bits in the mode argument supplied during calls to
/// the following functions:
///
/// open(), openat(), creat(), mkdir(), mkdirat(), mkfifo(), and mkfifoat()
/// RETURN VALUE
/// The file permission bits in the value returned by umask() shall be
/// the previous value of the file mode creation mask. The state of any
/// other bits in that value is unspecified, except that a subsequent
/// call to umask() with the returned value as cmask shall leave the state
/// of the mask the same as its state before the first call,
/// including any unspecified use of those bits.

pub fn sys_umask(cmask: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let umask = &mut scheduler.current_thread_group_mut().umask;

        let old = *umask;

        // Mask out all bits that are not file permissions bits.
        *umask = cmask & FileType::PERMISSIONS_MASK.bits() as mode_t;

        Ok(old as u32)
    })
}
