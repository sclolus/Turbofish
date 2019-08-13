use super::scheduler::SCHEDULER;
use super::SysResult;

/// The getgid() function shall return the effective group ID of the
/// calling process. The getgid() function shall not modify errno.
pub fn sys_getegid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_thread_group().credentials.egid
    }))
}
