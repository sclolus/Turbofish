//! tcsetpgrp syscall
use super::scheduler::SCHEDULER;
use super::Pid;
use super::SysResult;
use crate::terminal::TERMINAL;

/// If the process has a controlling terminal, tcsetpgrp() shall set
/// the foreground process group ID associated with the terminal to
/// pgid_id. The application shall ensure that the file associated
/// with fildes is the controlling terminal of the calling process and
/// the controlling terminal is currently associated with the session
/// of the calling process. The application shall ensure that the
/// value of pgid_id matches a process group ID of a process in the
/// same session as the calling process.
///
//TODO:
/// Attempts to use tcsetpgrp() from a process which is a member of a
/// background process group on a fildes associated with its
/// controlling terminal shall cause the process group to be sent a
/// SIGTTOU signal. If the calling thread is blocking SIGTTOU signals
/// or the process is ignoring SIGTTOU signals, the process shall be
/// allowed to perform the operation, and no signal is sent.
// TODO: file descriptor argument
pub fn sys_tcsetpgrp(_fildes: i32, pgid_id: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        let controlling_terminal = scheduler.current_thread_group().controlling_terminal;
        // dbg!(controlling_terminal);
        // dbg!(pgid_id);
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(controlling_terminal)
                .tcsetpgrp(pgid_id);
        }
    });
    Ok(0)
}
