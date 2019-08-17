use super::scheduler::SCHEDULER;
use super::SysResult;

pub unsafe fn sys_getppid() -> SysResult<u32> {
    Ok(unpreemptible_context!({
        SCHEDULER.lock().current_thread_group().parent as u32
    }))
}
