//! Close a file descriptor

use super::scheduler::SCHEDULER;
use super::SysResult;

pub fn sys_close(fd: i32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let fd_interface = &mut scheduler.current_thread_group_mut().fd_interface;

        fd_interface.close_fd(fd as u32)?;
    });
    Ok(0)
}
