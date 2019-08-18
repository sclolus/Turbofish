use super::SysResult;

use super::scheduler::{auto_preempt, Pid, SCHEDULER};
use super::signal_interface::JobAction;
use super::thread::Thread;

use core::convert::TryInto;
use errno::Errno;
use libc_binding::Signum;

/// The kill() function shall send a signal to a process or a group of
/// processes specified by pid. The signal to be sent is specified by
/// sig and is either one from the list given in <signal.h> or 0. If
/// sig is 0 (the null signal), error checking is performed but no
/// signal is actually sent. The null signal can be used to check the
/// validity of pid.
///
/// For a process to have permission to send a signal to a process
/// designated by pid, unless the sending process has appropriate
/// privileges, the real or effective user ID of the sending process
/// shall match the real or saved set-user-ID of the receiving
/// process.
///
/// If pid is greater than 0, sig shall be sent to the process whose
/// process ID is equal to pid.
///
/// If pid is 0, sig shall be sent to all processes (excluding an
/// unspecified set of system processes) whose process group ID is
/// equal to the process group ID of the sender, and for which the
/// process has permission to send a signal.
///
/// If pid is -1, sig shall be sent to all processes (excluding an
/// unspecified set of system processes) for which the process has
/// permission to send that signal.
///
/// If pid is negative, but not -1, sig shall be sent to all processes
/// (excluding an unspecified set of system processes) whose process
/// group ID is equal to the absolute value of pid, and for which the
/// process has permission to send a signal.
///
/// If the value of pid causes sig to be generated for the sending
/// process, and if sig is not blocked for the calling thread and if
/// no other thread has sig unblocked or is waiting in a sigwait()
/// function for sig, either sig or at least one pending unblocked
/// signal shall be delivered to the sending thread before kill()
/// returns.
///
/// The user ID tests described above shall not be applied when
/// sending SIGCONT to a process that is a member of the same session
/// as the sending process.
///
/// An implementation that provides extended security controls may
/// impose further implementation-defined restrictions on the sending
/// of signals, including the null signal. In particular, the system
/// may deny the existence of some or all of the processes specified
/// by pid.
///
/// The kill() function is successful if the process has permission to
/// send sig to any of the processes specified by pid. If kill()
/// fails, no signal shall be sent.
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
                // TODO: is this still necessary to autopreempt ?
                let _ignored_result = auto_preempt();
            }
        }
        Ok(0)
    })
}
