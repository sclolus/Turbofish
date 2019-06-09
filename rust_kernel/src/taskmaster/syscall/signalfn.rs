//! This file contain all the signal related syscall code

use super::SysResult;

use super::process::CpuState;
use super::scheduler::{Pid, SCHEDULER, SIGNAL_LOCK};
use super::signal::{SignalStatus, StructSigaction};

use core::convert::TryInto;
use errno::Errno;

#[repr(C)]
pub struct Sigaction {}

/// Register a new handler for a specified signum with sigaction params
pub unsafe fn sys_sigaction(signum: u32, act: *const StructSigaction, old_act: *mut StructSigaction) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let v = &mut scheduler.curr_process_mut().unwrap_running_mut().virtual_allocator;

        // Check if pointer exists in user virtual address space
        v.check_user_ptr::<StructSigaction>(act)?;
        if old_act as usize != 0 {
            v.check_user_ptr::<StructSigaction>(old_act)?;
        }
        // TODO: Use old_act
        scheduler
            .curr_process_mut()
            .signal
            .new_handler(signum.try_into().map_err(|_| Errno::Einval)?, act.as_ref().expect("Null PTR"))
    })
}

/// Send a signal to a specified PID process
pub unsafe fn sys_kill(pid: Pid, signum: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let task = scheduler.get_process_mut(pid).ok_or(Errno::Esrch)?;
        let res = task.signal.new_signal(signum.try_into().map_err(|_| Errno::Einval)?)?;

        let curr_process_pid = scheduler.curr_process_pid();
        // auto-sodo mode
        if curr_process_pid == pid {
            let signal = scheduler.curr_process_mut().signal.check_pending_signals();
            if let Some(SignalStatus::Deadly(_signum)) | Some(SignalStatus::Handled(_signum)) = signal {
                SIGNAL_LOCK = true;
            }
        }
        Ok(res)
    })
}

/// Register a new handler for a specified signum
pub unsafe fn sys_signal(signum: u32, handler: extern "C" fn(i32)) -> SysResult<u32> {
    unpreemptible_context!({
        let s: StructSigaction = StructSigaction {
            sa_handler: handler as usize,
            sa_mask: Default::default(),
            sa_flags: 0, // TODO: Default for sys_signal is SA_RESTART
            sa_restorer: 0,
        };

        let mut scheduler = SCHEDULER.lock();
        scheduler.curr_process_mut().signal.new_handler(signum.try_into().map_err(|_| Errno::Einval)?, &s)
    })
}

/// Must know who is the last pending signal
/// Decrease signal frame and POP signal in list
/// Terminate the last pending signal
pub unsafe fn sys_sigreturn(cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        scheduler.curr_process_mut().signal.terminate_pending_signal(cpu_state as u32);
        Ok((*cpu_state).registers.eax)
    })
}
