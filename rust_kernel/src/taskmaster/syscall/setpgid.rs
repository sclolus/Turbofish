use super::scheduler::{Pid, SCHEDULER};
use super::SysResult;
use errno::Errno;

pub fn sys_setpgid(pid: Pid, pgid: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        if pid == 0 {
            scheduler.current_thread_group_mut().pgid = pgid;
            Ok(0)
        } else {
            scheduler
                .get_thread_group_mut(pid)
                .ok_or(Errno::Esrch)?
                .pgid = pgid;
            Ok(0)
        }
    })
}
