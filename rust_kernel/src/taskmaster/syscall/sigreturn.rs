use super::SysResult;

use super::process::CpuState;
use super::scheduler::SCHEDULER;

/// Must know who is the last pending signal
/// Decrease signal frame and POP signal in list
/// Terminate the last pending signal
pub unsafe fn sys_sigreturn(cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        scheduler
            .current_thread_mut()
            .signal
            .terminate_pending_signal(cpu_state as u32);
        Ok((*cpu_state).registers.eax)
    })
}
