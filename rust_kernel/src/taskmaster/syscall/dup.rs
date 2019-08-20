//! This file contains the description of the dup syscall

use super::scheduler::SCHEDULER;
use super::SysResult;

/// Duplicate a file descriptor
pub fn sys_dup(old_fd: u32) -> SysResult<u32> {
    let ret = unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let fd_interface = &mut scheduler.current_thread_group_mut().fd_interface;

        fd_interface.dup(old_fd)?
    });
    Ok(ret)
}
