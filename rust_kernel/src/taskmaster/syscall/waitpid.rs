//! waitpid (wait) implementations

use super::scheduler::Pid;
use super::scheduler::SCHEDULER;
use super::scheduler::{interruptible, uninterruptible};
use super::SysResult;

use errno::Errno;

pub unsafe fn sys_waitpid(pid: i32, wstatus: *mut i32, options: i32) -> SysResult<u32> {
    uninterruptible();
    // let res = SCHEDULER.lock().wait();
    interruptible();
    Ok(0)
}
