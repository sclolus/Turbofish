//! sys_geteuid()

use super::scheduler::SCHEDULER;
use super::SysResult;

/// The geteuid() function shall return the effective user ID of the
/// calling process. The geteuid() function shall not modify errno.
pub fn sys_geteuid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_thread_group().credentials.euid
    }))
}
