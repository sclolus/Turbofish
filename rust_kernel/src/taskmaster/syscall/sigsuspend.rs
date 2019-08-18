use super::SysResult;
use super::{sys_pause, sys_sigprocmask};

use super::signal_interface::sigset_t;
use libc_binding::SIG_SETMASK;

pub unsafe fn sys_sigsuspend(sigmask: *const sigset_t) -> SysResult<u32> {
    let mut oldmask: sigset_t = 0;
    sys_sigprocmask(SIG_SETMASK, sigmask, &mut oldmask)?;
    let ret = sys_pause();
    sys_sigprocmask(SIG_SETMASK, &oldmask, 0 as *mut sigset_t)?;
    ret
}
