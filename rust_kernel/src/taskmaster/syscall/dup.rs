//! This file contains the description of the dup syscall

use super::scheduler::SCHEDULER;
use super::SysResult;

/// Duplicate a file descriptor
pub fn sys_dup(old_fd: u32) -> SysResult<u32> {
    let ret = unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let task = scheduler.current_thread_mut();

        task.fd_interface.dup(old_fd)?
    });
    Ok(ret)
}
