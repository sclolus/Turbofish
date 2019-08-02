//! This file contain all the signal related syscall code

use super::SysResult;

use super::process::CpuState;
use super::scheduler::{auto_preempt, Pid, SCHEDULER, SIGNAL_LOCK};
use super::signal::{JobAction, StructSigaction};
use super::task::WaitingState;

use core::convert::TryInto;
use errno::Errno;

/// Register a new handler for a specified signum with sigaction params
pub unsafe fn sys_sigaction(signum: u32, act: *const StructSigaction, old_act: *mut StructSigaction) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let v = &mut scheduler.current_task_mut().unwrap_process_mut().virtual_allocator;

        // Check if pointer exists in user virtual address space
        v.check_user_ptr::<StructSigaction>(act)?;
        if old_act as usize != 0 {
            v.check_user_ptr::<StructSigaction>(old_act)?;
        }
        // TODO: Use old_act
        scheduler
            .current_task_mut()
            .signal
            .new_handler(signum.try_into().map_err(|_| Errno::Einval)?, act.as_ref().expect("Null PTR"))
    })
}

/// Send a signal to a specified PID process
pub unsafe fn sys_kill(pid: Pid, signum: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let current_task_pid = scheduler.current_task_pid();
        let task = scheduler.get_process_mut(pid).ok_or(Errno::Esrch)?;
        let signum = signum.try_into().map_err(|_| Errno::Einval)?;
        let res = task.signal.generate_signal(signum)?;

        // auto-sodo mode
        if current_task_pid == pid {
            let action = task.signal.get_job_action();

            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                // Auto-preempt calling in case of Self stop
                auto_preempt();
            } else if action.intersects(JobAction::TERMINATE) || action.intersects(JobAction::INTERRUPT) {
                SIGNAL_LOCK = true;
            }
        }
        Ok(res)
    })
}

/// Wait for signal
pub unsafe fn sys_pause() -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER.lock().current_task_mut().set_waiting(WaitingState::Pause);
        /*
         * pause() returns only when a signal was caught and the signal-catching function returned.
         * In this case, pause() returns -1, and errno is set to EINTR
         */
        auto_preempt();
        Err(Errno::Eintr)
    })
}

/// Register a new handler for a specified signum
pub unsafe fn sys_signal(signum: u32, handler: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let s: StructSigaction = StructSigaction {
            sa_handler: handler,
            sa_mask: Default::default(),
            sa_flags: Default::default(),
            sa_restorer: 0,
        };

        let mut scheduler = SCHEDULER.lock();
        scheduler.current_task_mut().signal.new_handler(signum.try_into().map_err(|_| Errno::Einval)?, &s)
    })
}

/// Must know who is the last pending signal
/// Decrease signal frame and POP signal in list
/// Terminate the last pending signal
pub unsafe fn sys_sigreturn(cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        scheduler.current_task_mut().signal.terminate_pending_signal(cpu_state as u32);
        Ok((*cpu_state).registers.eax)
    })
}
