use super::SysResult;

use super::scheduler::{auto_preempt, SCHEDULER};
use super::thread::WaitingState;

use libc_binding::Errno;

/// The pause() function shall suspend the calling thread until
/// delivery of a signal whose action is either to execute a
/// signal-catching function or to terminate the process.
///
/// If the action is to terminate the process, pause() shall not
/// return.
///
/// If the action is to execute a signal-catching function, pause()
/// shall return after the signal-catching function returns.
///
/// Since pause() suspends thread execution indefinitely unless
/// interrupted by a signal, there is no successful completion return
/// value. A value of -1 shall be returned and errno set to indicate
/// the error.
pub unsafe fn sys_pause() -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER
            .lock()
            .current_thread_mut()
            .set_waiting(WaitingState::Pause);
        /*
         * pause() returns only when a signal was caught and the
         * signal-catching function returned.  In this case, pause()
         * returns -1, and errno is set to EINTR
         */
        let _ignored_result = auto_preempt();
        Err(Errno::EINTR)
    })
}
