use super::scheduler::SCHEDULER;
use super::SysResult;

/// The getpid() function shall return the process ID of the calling
/// process.
pub unsafe fn sys_getpid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_task_id().0 as u32
    }))
}
