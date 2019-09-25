//! sys_seteuid()

use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::uid_t;
use libc_binding::Errno;

/// If uid is equal to the real user ID or the saved set-user-ID, or
/// if the process has appropriate privileges, seteuid() shall set the
/// effective user ID of the calling process to uid; the real user ID
/// and saved set-user-ID shall remain unchanged.
///
/// The seteuid() function shall not affect the supplementary group
/// list in any way.
pub fn sys_seteuid(uid: uid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.euid == 0 || (uid == cred.euid || uid == cred.suid) {
            cred.euid = uid;
            Ok(0)
        } else {
            Err(Errno::EPERM)
        }
    })
}
