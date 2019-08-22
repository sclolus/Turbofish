use super::scheduler::{Pid, SCHEDULER};
use super::SysResult;
use libc_binding::Errno;

/// The setpgid() function shall either join an existing process group
/// or create a new process group within the session of the calling
/// process.
///
/// The process group ID of a session leader shall not change.
///
/// Upon successful completion, the process group ID of the process
/// with a process ID that matches pid shall be set to pgid.
///
/// As a special case, if pid is 0, the process ID of the calling
/// process shall be used. Also, if pgid is 0, the process ID of the
/// indicated process shall be used.
pub fn sys_setpgid(mut pid: Pid, mut pgid: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        if pid == 0 {
            pid = scheduler.current_task_id().0
        }
        if pgid == 0 {
            pgid = pid
        }
        scheduler
            .get_thread_group_mut(pid)
            .ok_or(Errno::ESRCH)?
            .pgid = pgid;

        Ok(0)
    })
}
