//! sys_getuid()

use super::scheduler::SCHEDULER;
use super::SysResult;

/// The getuid() function shall return the real user ID of the calling
/// process. The getuid() function shall not modify errno.
pub fn sys_getuid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_thread_group().credentials.uid as u32
    }))
}
