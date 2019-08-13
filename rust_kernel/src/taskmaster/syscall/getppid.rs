use super::scheduler::SCHEDULER;
use super::SysResult;

pub unsafe fn sys_getppid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_task().parent.unwrap_or(1) as u32
    }))
}
