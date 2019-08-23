//! waitpid (wait) implementations

use super::scheduler::{auto_preempt, unpreemptible};
use super::scheduler::{Scheduler, SCHEDULER};
use super::thread::{AutoPreemptReturnValue, WaitingState};
use super::thread_group::{JobState, Status};
use super::SysResult;

use libc_binding::{Errno, Pid, WCONTINUED, WNOHANG, WUNTRACED};

/// The wait() and waitpid() functions shall obtain status information
/// (see Status Information) pertaining to one of the caller's child
/// processes. The wait() function obtains status information for
/// process termination from any child process. The waitpid() function
/// obtains status information for process termination, and optionally
/// process stop and/or continue, from a specified subset of the child
/// processes.
///
/// The wait() function shall cause the calling thread to become
/// blocked until status information generated by child process
/// termination is made available to the thread, or until delivery of
/// a signal whose action is either to execute a signal-catching
/// function or to terminate the process, or an error occurs. If
/// termination status information is available prior to the call to
/// wait(), return shall be immediate. If termination status
/// information is available for two or more child processes, the
/// order in which their status is reported is unspecified.
///
/// As described in Status Information, the wait() and waitpid()
/// functions consume the status information they obtain.
///
/// The behavior when multiple threads are blocked in wait(),
/// waitid(), or waitpid() is described in Status Information.
///
/// The waitpid() function shall be equivalent to wait() if the pid
/// argument is (pid_t)-1 and the options argument is 0. Otherwise,
/// its behavior shall be modified by the values of the pid and
/// options arguments.
///
/// The options argument is constructed from the bitwise-inclusive OR
/// of zero or more of the following flags, defined in the
/// <sys/wait.h> header:
///
//TODO:
/// WCONTINUED [XSI] [Option Start] The waitpid() function shall
///     report the status of any continued child process specified by
///     pid whose status has not been reported since it continued from
///     a job control stop. [Option End]
/// WNOHANG The waitpid() function shall not suspend execution of the
///     calling thread if status is not immediately available for one
///     of the child processes specified by pid.
/// WUNTRACED The status of any child processes specified by pid that
///     are stopped, and whose status has not yet been reported since
///     they stopped, shall also be reported to the requesting
///     process.
///
/// If wait() or waitpid() return because the status of a child
/// process is available, these functions shall return a value equal
/// to the process ID of the child process. In this case, if the value
/// of the argument stat_loc is not a null pointer, information shall
/// be stored in the location pointed to by stat_loc. The value stored
/// at the location pointed to by stat_loc shall be 0 if and only if
/// the status returned is from a terminated child process that
/// terminated by one of the following means:
///
///     The process returned 0 from main().
///
///     The process called _exit() or exit() with a status argument of
///     0.
///
///     The process was terminated because the last thread in the
///     process terminated.
///
/// Regardless of its value, this information may be interpreted using
/// the following macros, which are defined in <sys/wait.h> and
/// evaluate to integral expressions; the stat_val argument is the
/// integer value pointed to by stat_loc.
///
/// WIFEXITED(stat_val) Evaluates to a non-zero value if status was
///     returned for a child process that terminated normally.
/// WEXITSTATUS(stat_val) If the value of WIFEXITED(stat_val) is
///     non-zero, this macro evaluates to the low-order 8 bits of the
///     status argument that the child process passed to _exit() or
///     exit(), or the value the child process returned from main().
/// WIFSIGNALED(stat_val) Evaluates to a non-zero value if status
///     was returned for a child process that terminated due to the
///     receipt of a signal that was not caught (see <signal.h>).
/// WTERMSIG(stat_val) If the value of WIFSIGNALED(stat_val) is
///     non-zero, this macro evaluates to the number of the signal
///     that caused the termination of the child process.
/// WIFSTOPPED(stat_val) Evaluates to a non-zero value if status
///     was returned for a child process that is currently stopped.
/// WSTOPSIG(stat_val) If the value of WIFSTOPPED(stat_val) is
///     non-zero, this macro evaluates to the number of the signal
///     that caused the child process to stop.
/// WIFCONTINUED(stat_val)
///     [XSI] [Option Start] Evaluates to a non-zero value if status
///     was returned for a child process that has continued from a job
///     control stop. [Option End]
///
/// [SPN] [Option Start] It is unspecified whether the status value
/// returned by calls to wait() or waitpid() for processes created by
/// posix_spawn() or posix_spawnp() can indicate a
/// WIFSTOPPED(stat_val) before subsequent calls to wait() or
/// waitpid() indicate WIFEXITED(stat_val) as the result of an error
/// detected before the new process image starts executing.
///
/// It is unspecified whether the status value returned by calls to
/// wait() or waitpid() for processes created by posix_spawn() or
/// posix_spawnp() can indicate a WIFSIGNALED(stat_val) if a signal is
/// sent to the parent's process group after posix_spawn() or
/// posix_spawnp() is called. [Option End]
///
/// If the information pointed to by stat_loc was stored by a call to
/// waitpid() that specified the WUNTRACED flag [XSI] [Option Start]
/// and did not specify the WCONTINUED flag, [Option End] exactly one
/// of the macros WIFEXITED(*stat_loc), WIFSIGNALED(*stat_loc), and
/// WIFSTOPPED(*stat_loc) shall evaluate to a non-zero value.
///
/// [XSI] [Option Start] If the information pointed to by stat_loc was
/// stored by a call to waitpid() that specified the WUNTRACED and
/// WCONTINUED flags, exactly one of the macros WIFEXITED(*stat_loc),
/// WIFSIGNALED(*stat_loc), WIFSTOPPED(*stat_loc), and
/// WIFCONTINUED(*stat_loc) shall evaluate to a non-zero
/// value. [Option End]
///
/// If the information pointed to by stat_loc was stored by a call to
/// waitpid() that did not specify the WUNTRACED [XSI] [Option Start]
/// or WCONTINUED [Option End] flags, or by a call to the wait()
/// function, exactly one of the macros WIFEXITED(*stat_loc) and
/// WIFSIGNALED(*stat_loc) shall evaluate to a non-zero value.
///
/// [XSI] [Option Start] If the information pointed to by stat_loc was
/// stored by a call to waitpid() that did not specify the WUNTRACED
/// flag and specified the WCONTINUED flag, exactly one of the macros
/// WIFEXITED(*stat_loc), WIFSIGNALED(*stat_loc), and
/// WIFCONTINUED(*stat_loc) shall evaluate to a non-zero
/// value. [Option End]
///
/// If _POSIX_REALTIME_SIGNALS is defined, and the implementation
/// queues the SIGCHLD signal, then if wait() or waitpid() returns
/// because the status of a child process is available, any pending
/// SIGCHLD signal associated with the process ID of the child process
/// shall be discarded. Any other pending SIGCHLD signals shall remain
/// pending.
///
/// Otherwise, if SIGCHLD is blocked, if wait() or waitpid() return
/// because the status of a child process is available, any pending
/// SIGCHLD signal shall be cleared unless the status of another child
/// process is available.
///
/// For all other conditions, it is unspecified whether child status
/// will be available when a SIGCHLD signal is delivered.
///
/// There may be additional implementation-defined circumstances under
/// which wait() or waitpid() report status. This shall not occur
/// unless the calling process or one of its child processes
/// explicitly makes use of a non-standard extension. In these cases
/// the interpretation of the reported status is
/// implementation-defined.
///
/// If a parent process terminates without waiting for all of its
/// child processes to terminate, the remaining child processes shall
/// be assigned a new parent process ID corresponding to an
/// implementation-defined system process.
///
///RETURN VALUE
///
/// If wait() or waitpid() returns because the status of a child
/// process is available, these functions shall return a value equal
/// to the process ID of the child process for which status is
/// reported. If wait() or waitpid() returns due to the delivery of a
/// signal to the calling process, -1 shall be returned and errno set
/// to [EINTR]. If waitpid() was invoked with WNOHANG set in options,
/// it has at least one child process specified by pid for which
/// status is not available, and status is not available for any
/// process specified by pid, 0 is returned. Otherwise, -1 shall be
/// returned, and errno set to indicate the error.
///
///ORS
///
/// The wait() function shall fail if:
///
/// [ECHILD] The calling process has no existing unwaited-for child
///     processes.  [EINTR] The function was interrupted by a
///     signal. The value of the location pointed to by stat_loc is
///     undefined.
///
/// The waitpid() function shall fail if:
///
/// [ECHILD] The process specified by pid does not exist or is not a
///     child of the calling process, or the process group specified
///     by pid does not exist or does not have any member process that
///     is a child of the calling process.  [EINTR] The function was
///     interrupted by a signal. The value of the location pointed to
///     by stat_loc is undefined.  [EINVAL] The options argument is
///     not valid.
fn waitpid(pid: i32, wstatus: *mut i32, options: u32) -> SysResult<u32> {
    let mut scheduler = SCHEDULER.lock();

    {
        let v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();

        // If wstatus is not NULL, wait() and waitpid() store status information in the int to which it points.
        // If the given pointer is a bullshit pointer, wait() and waitpid() return EFAULT
        if wstatus != 0x0 as *mut i32 {
            v.check_user_ptr::<i32>(wstatus)?;
        }
    }

    // WIFEXITED(wstatus)
    // returns true if the child terminated normally, that is, by calling exit(3) or _exit(2), or by returning from main().

    // WEXITSTATUS(wstatus)
    // returns  the exit status of the child. This consists of the least significant 8 bits of the status argument that
    // the child specified in a call to exit(3) or _exit(2) or as the argument for a
    // return statement in main().  This macro should be employed only if WIFEXITED returned true.

    // the two next macro are signal dedicated ... WIFSIGNALED(wstatus) && WTERMSIG(wstatus)

    // Return EINVAL for any unknown option
    // TODO: Code at least WNOHANG and WUNTRACED for Posix
    if options > 7 {
        return Err(Errno::EINVAL);
    }

    let thread_group = scheduler.current_thread_group();

    // The pid argument specifies a set of child processes for which
    // status is requested. The waitpid() function shall only return the
    // status of a child process from this set:
    // child_pid is a Option. Some(child_pid) if some child is dead, None
    // otherwise
    let child_pid = match pid {
        // If pid is equal to (pid_t)-1, status is requested for any
        // child process. In this respect, waitpid() is then
        // equivalent to wait().
        -1 => {
            // Check if at leat one child exists
            if thread_group.unwrap_running().child.len() == 0 {
                return Err(Errno::ECHILD);
            }
            // Check is the at least one child is a already a zombie -> Return immediatly child PID
            thread_group
                .unwrap_running()
                .child
                .iter()
                .find(|&current_pid| has_status_available(&*scheduler, *current_pid, options))
        }
        // If pid is less than (pid_t)-1, status is requested for any
        // child process whose process group ID is equal to the
        // absolute value of pid.
        mut pid if (pid < 0 || pid == 0) => {
            // If pid is 0, status is requested for any child process
            // whose process group ID is equal to that of the calling
            // process.
            if pid == 0 {
                pid = thread_group.pgid;
            }
            // TODO: can be optim
            let candidate_number = thread_group
                .unwrap_running()
                .child
                .iter()
                .map(|&current_pid| {
                    scheduler
                        .get_thread_group(current_pid)
                        .expect("Pid must be here")
                })
                .filter(|tg| tg.pgid == -pid)
                .count();

            if candidate_number == 0 {
                return Err(Errno::ECHILD);
            }

            thread_group
                .unwrap_running()
                .child
                .iter()
                .find(|&current_pid| has_status_available(&*scheduler, *current_pid, options))
        }
        // If pid is greater than 0, it specifies the process ID of a
        // single child process for which status is requested.
        pid if pid > 0 => {
            // Check if specified child exists
            if let Some(elem) = thread_group
                .unwrap_running()
                .child
                .iter()
                .find(|&&current_pid| current_pid == pid)
            {
                if has_status_available(&*scheduler, *elem, options) {
                    Some(elem)
                } else {
                    None
                }
            } else {
                return Err(Errno::ECHILD);
            }
        }
        _ => unreachable!(),
    };

    match child_pid {
        Some(&dead_pid) => {
            let tg = scheduler
                .get_thread_group_mut(dead_pid)
                .expect("Pid must be here");

            let status = match tg.get_death_status() {
                Some(status) => {
                    dbg!(dead_pid);
                    scheduler.current_thread_group_mut().remove_child(dead_pid);
                    scheduler.remove_thread_group(dead_pid);
                    status
                }
                None => Status::from(tg.job.consume_last_event().expect("no status")).into(),
            };
            // TODO: Manage terminated value with signal
            if wstatus != 0x0 as *mut i32 {
                unsafe { *wstatus = status }
            }
            // fflush zombie
            // Return immediatly
            Ok(dead_pid as u32)
        }
        None if options & WNOHANG != 0 => {
            return Ok(0);
        }
        None => {
            // Set process as Waiting for ChildDeath. set the PID option inside
            let pgid = thread_group.pgid;
            scheduler
                .current_thread_mut()
                .set_waiting(WaitingState::Waitpid { pid, pgid, options });

            let ret = auto_preempt()?;

            // Re-Lock immediatly critical ressources (auto_preempt unlocked all)
            unpreemptible();
            let mut scheduler = SCHEDULER.lock();

            let child_pid = match ret {
                AutoPreemptReturnValue::Wait {
                    dead_process_pid,
                    status,
                } => {
                    if status.is_terminated() {
                        let thread_group = scheduler.current_thread_group_mut();
                        thread_group.remove_child(dead_process_pid);
                        scheduler.remove_thread_group(dead_process_pid);
                    }
                    // Set wstatus pointer is not null by reading y
                    if wstatus != 0x0 as *mut i32 {
                        unsafe {
                            *wstatus = status.into();
                        }
                    }
                    dead_process_pid
                }
                _ => panic!("WTF"),
            };
            Ok(child_pid as u32)
        }
    }
}

fn has_status_available(scheduler: &Scheduler, pid: Pid, options: u32) -> bool {
    let thread_group = scheduler.get_thread_group(pid).expect("Pid must be here");
    thread_group.is_zombie()
        || (options & WUNTRACED != 0 && thread_group.job.get_lat_event() == Some(JobState::Stopped))
        || (options & WCONTINUED != 0
            && thread_group.job.get_lat_event() == Some(JobState::Continued))
}

pub fn sys_waitpid(pid: i32, wstatus: *mut i32, options: u32) -> SysResult<u32> {
    unpreemptible_context!({ waitpid(pid, wstatus, options) })
}
