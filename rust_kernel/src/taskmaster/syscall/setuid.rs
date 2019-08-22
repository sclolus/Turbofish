//! sys_setuid()

use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::uid_t;
use libc_binding::Errno;

/// If the process has appropriate privileges, setuid() shall set the
/// real user ID, effective user ID, and the saved set-user-ID of the
/// calling process to uid.
///
/// If the process does not have appropriate privileges, but uid is
/// equal to the real user ID or the saved set-user-ID, setuid() shall
/// set the effective user ID to uid; the real user ID and saved
/// set-user-ID shall remain unchanged.
///
/// The setuid() function shall not affect the supplementary group
/// list in any way.
///
/// The setuid() function shall fail, return -1, and set errno to the
/// corresponding value if one or more of the following are true:
/// [EINVAL] The value of the uid argument is invalid and not
///   supported by the implementation.
/// [EPERM] The process does not
///   have appropriate privileges and uid does not match the real user
///   ID or the saved set-user-ID.

pub fn sys_setuid(uid: uid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.uid == 0 {
            cred.uid = uid;
            cred.euid = uid;
            cred.suid = uid;
            Ok(0)
        } else if uid == cred.uid || uid == cred.suid {
            cred.euid = uid;
            Ok(0)
        } else {
            Err(Errno::EPERM)
        }
    })
}
