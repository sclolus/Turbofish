use super::scheduler::SCHEDULER;
use super::SysResult;
use errno::Errno;
use libc_binding::gid_t;

/// If the process has appropriate privileges, setgid() shall set the
/// real group ID, effective group ID, and the saved set-group-ID of
/// the calling process to gid.
///
/// If the process does not have appropriate privileges, but gid is
/// equal to the real group ID or the saved set-group-ID, setgid()
/// shall set the effective group ID to gid; the real group ID and
/// saved set-group-ID shall remain unchanged.
///
/// The setgid() function shall not affect the supplementary group
/// list in any way.
///
/// Any supplementary group IDs of the calling process shall remain
/// unchanged.
///
/// The setgid() function shall fail if:
/// [EINVAL] The value of the gid argument is invalid and is not
///     supported by the implementation.
/// [EPERM] The process does not
///     have appropriate privileges and gid does not match the real
///     group ID or the saved set-group-ID.

pub fn sys_setgid(gid: gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.uid == 0 {
            cred.gid = gid;
            cred.egid = gid;
            cred.sgid = gid;
            Ok(0)
        } else if gid == cred.uid || gid == cred.sgid {
            cred.egid = gid;
            Ok(0)
        } else {
            Err(Errno::Eperm)
        }
    })
}
