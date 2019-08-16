use super::scheduler::{Pid, SCHEDULER};
use super::SysResult;
use errno::Errno;

pub fn sys_getpgid(pid: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        if pid == 0 {
            Ok(scheduler.current_thread_group().pgid as u32)
        } else {
            Ok(scheduler.get_thread_group(pid).ok_or(Errno::Esrch)?.pgid as u32)
        }
    })
}
