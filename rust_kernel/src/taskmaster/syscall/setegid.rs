use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::gid_t;
use libc_binding::Errno;

/// If gid is equal to the real group ID or the saved set-group-ID, or
/// if the process has appropriate privileges, setegid() shall set the
/// effective group ID of the calling process to gid; the real group
/// ID, saved set-group-ID, and any supplementary group IDs shall
/// remain unchanged.
///
/// The setegid() function shall not affect the supplementary group
/// list in any way.
pub fn sys_setegid(gid: gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.euid == 0 || (gid == cred.egid || gid == cred.sgid) {
            cred.egid = gid;
            Ok(0)
        } else {
            Err(Errno::EPERM)
        }
    })
}
