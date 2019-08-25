use super::SysResult;
use super::{sys_pause, sys_sigprocmask};

use super::signal_interface::sigset_t;
use libc_binding::SIG_SETMASK;

/// The sigsuspend() function shall replace the current signal mask of
/// the calling thread with the set of signals pointed to by sigmask
/// and then suspend the thread until delivery of a signal whose
/// action is either to execute a signal-catching function or to
/// terminate the process. This shall not cause any other signals that
/// may have been pending on the process to become pending on the
/// thread.
///
/// If the action is to terminate the process then sigsuspend() shall
/// never return. If the action is to execute a signal-catching
/// function, then sigsuspend() shall return after the signal-catching
/// function returns, with the signal mask restored to the set that
/// existed prior to the sigsuspend() call.
///
/// It is not possible to block signals that cannot be ignored. This
/// is enforced by the system without causing an error to be
/// indicated.
pub fn sys_sigsuspend(sigmask: *const sigset_t) -> SysResult<u32> {
    let mut oldmask: sigset_t = 0;
    sys_sigprocmask(SIG_SETMASK, sigmask, &mut oldmask)?;
    let ret = sys_pause();
    sys_sigprocmask(SIG_SETMASK, &oldmask, 0 as *mut sigset_t)?;
    ret
}
