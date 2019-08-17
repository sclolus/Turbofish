use super::SysResult;

use super::scheduler::{auto_preempt, Pid, SCHEDULER};
use super::signal_interface::JobAction;
use super::thread::Thread;
use libc_binding::Signum;

use core::convert::TryInto;
use errno::Errno;

/// Send a signal to a specified PID process
pub unsafe fn sys_kill(pid: i32, signum: u32) -> SysResult<u32> {
    fn generate_signal<'a, T: Iterator<Item = &'a mut Thread>>(
        iter: T,
        signum: Signum,
    ) -> SysResult<u32> {
        let mut present = false;
        for thread in iter {
            present = true;
            thread.signal.generate_signal(signum)?;
        }
        if !present {
            return Err(Errno::Esrch);
        }
        Ok(0)
    }
    unpreemptible_context!({
        let signum = signum.try_into().map_err(|_| Errno::Einval)?;
        let mut scheduler = SCHEDULER.lock();

        if pid < -1 {
            generate_signal(
                scheduler
                    .iter_thread_groups_mut()
                    .filter_map(|thread_group| {
                        if thread_group.pgid == -pid as Pid {
                            thread_group.get_first_thread()
                        } else {
                            None
                        }
                    }),
                signum,
            )
        } else if pid == -1 {
            generate_signal(
                scheduler
                    .iter_thread_groups_mut()
                    .filter_map(|thread_group| thread_group.get_first_thread()),
                signum,
            )
        } else {
            generate_signal(
                scheduler.get_thread_mut((pid as Pid, 0)).into_iter(),
                signum,
            )
        }?;
        // auto-sodo mode
        let current_thread_pid = scheduler.current_task_id().0;
        if (pid > 0 && current_thread_pid == pid)
            || (pid < -1 && scheduler.current_thread_group().pgid == -pid as Pid)
            || pid == -1
        {
            let thread = scheduler.current_thread();
            let action = thread.signal.get_job_action();

            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                // Auto-preempt calling in case of Self stop
                auto_preempt();
            }
        }
        Ok(0)
    })
}
