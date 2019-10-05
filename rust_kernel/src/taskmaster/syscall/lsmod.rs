//! sys_lsmod

use super::scheduler::SCHEDULER;
use super::SysResult;

/// List all locaded kernel module
pub fn sys_lsmod() -> SysResult<u32> {
    unpreemptible_context!({ SCHEDULER.lock().list_modules() })
}
