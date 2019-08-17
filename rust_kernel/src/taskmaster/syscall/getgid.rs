//! sys_getgid()

use super::scheduler::SCHEDULER;
use super::SysResult;

/// The getgid() function shall return the real group ID of the
/// calling process. The getgid() function shall not modify errno.
pub fn sys_getgid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_thread_group().credentials.gid
    }))
}
