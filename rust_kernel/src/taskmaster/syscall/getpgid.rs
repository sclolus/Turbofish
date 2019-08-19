use super::scheduler::{Pid, SCHEDULER};
use super::SysResult;
use libc_binding::Errno;

/// The getpgid() function shall return the process group ID of the
/// process whose process ID is equal to pid. If pid is equal to 0,
/// getpgid() shall return the process group ID of the calling
/// process.
pub fn sys_getpgid(pid: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        if pid == 0 {
            Ok(scheduler.current_thread_group().pgid as u32)
        } else {
            Ok(scheduler.get_thread_group(pid).ok_or(Errno::ESRCH)?.pgid as u32)
        }
    })
}
