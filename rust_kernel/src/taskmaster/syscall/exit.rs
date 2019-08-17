use super::scheduler::{unpreemptible, SCHEDULER};

/// Exit from a process
pub unsafe fn sys_exit(status: i32) -> ! {
    unpreemptible();
    SCHEDULER.lock().current_thread_group_exit(status);
}
