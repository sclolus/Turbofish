use super::SysResult;

use super::scheduler::{auto_preempt, SCHEDULER};
use super::thread::WaitingState;

use errno::Errno;
/// Wait for signal
pub unsafe fn sys_pause() -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER
            .lock()
            .current_thread_mut()
            .set_waiting(WaitingState::Pause);
        /*
         * pause() returns only when a signal was caught and the signal-catching function returned.
         * In this case, pause() returns -1, and errno is set to EINTR
         */
        auto_preempt();
        Err(Errno::Eintr)
    })
}
