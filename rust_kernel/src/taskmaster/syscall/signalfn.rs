//! This file contain all the signal related syscall code

use super::SysResult;

use super::process::CpuState;
use super::scheduler::{auto_preempt, Pid, SCHEDULER, SIGNAL_LOCK};
use super::signal::{sigset_t, JobAction, SignalInterface, Signum, StructSigaction};
use super::task::{Task, WaitingState};

use core::convert::TryInto;
use errno::Errno;

/// Register a new handler for a specified signum with sigaction params
pub unsafe fn sys_sigaction(
    signum: u32,
    act: *const StructSigaction,
    old_act: *mut StructSigaction,
) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        {
            let v = scheduler
                .current_task_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr::<StructSigaction>(act)?;
            if old_act as usize != 0 {
                v.check_user_ptr::<StructSigaction>(old_act)?;
            }
        }
        // TODO: Use old_act
        *old_act = scheduler.current_task_mut().signal.new_handler(
            signum.try_into().map_err(|_| Errno::Einval)?,
            act.as_ref().expect("Null PTR"),
        )?;
        Ok(0)
    })
}

/// Send a signal to a specified PID process
pub unsafe fn sys_kill(pid: i32, signum: u32) -> SysResult<u32> {
    fn generate_signal<'a, T: Iterator<Item = &'a mut Task>>(
        iter: T,
        signum: Signum,
    ) -> SysResult<u32> {
        let mut present = false;
        for task in iter {
            present = true;
            task.signal.generate_signal(signum)?;
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
                    .all_process
                    .iter_mut()
                    .filter_map(|(_pid, thread_group)| {
                        if thread_group.pgid == -pid as Pid {
                            Some(thread_group.get_first_thread())
                        } else {
                            None
                        }
                    }),
                signum,
            )
        } else if pid == -1 {
            generate_signal(
                scheduler
                    .all_process
                    .iter_mut()
                    .filter_map(|(_pid, thread_group)| Some(thread_group.get_first_thread())),
                signum,
            )
        } else {
            generate_signal(scheduler.get_task_mut((pid as Pid, 0)).into_iter(), signum)
        }?;
        // auto-sodo mode
        let current_task_pid = scheduler.current_task_id().0;
        if (pid > 0 && current_task_pid == pid as u32)
            || (pid < -1 && scheduler.current_thread_group().pgid == -pid as Pid)
            || pid == -1
        {
            let task = scheduler.current_task();
            let action = task.signal.get_job_action();

            if action.intersects(JobAction::STOP) && !action.intersects(JobAction::TERMINATE) {
                // Auto-preempt calling in case of Self stop
                auto_preempt();
            } else if action.intersects(JobAction::TERMINATE)
                || action.intersects(JobAction::INTERRUPT)
            {
                SIGNAL_LOCK = true;
            }
        }
        Ok(0)
    })
}

/// Wait for signal
pub unsafe fn sys_pause() -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER
            .lock()
            .current_task_mut()
            .set_waiting(WaitingState::Pause);
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
        let struct_sigaction = scheduler
            .current_task_mut()
            .signal
            .new_handler(signum.try_into().map_err(|_| Errno::Einval)?, &s)?;
        Ok(struct_sigaction.sa_handler as u32)
    })
}

/// Must know who is the last pending signal
/// Decrease signal frame and POP signal in list
/// Terminate the last pending signal
pub unsafe fn sys_sigreturn(cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        scheduler
            .current_task_mut()
            .signal
            .terminate_pending_signal(cpu_state as u32);
        Ok((*cpu_state).registers.eax)
    })
}

pub unsafe fn sys_sigsuspend(sigmask: *const sigset_t) -> SysResult<u32> {
    let mut oldmask: sigset_t = 0;
    sys_sigprocmask(SignalInterface::SIG_SETMASK, sigmask, &mut oldmask)?;
    let ret = sys_pause();
    sys_sigprocmask(SignalInterface::SIG_SETMASK, &oldmask, 0 as *mut sigset_t)?;
    ret
}

pub unsafe fn sys_sigprocmask(
    how: i32,
    set: *const sigset_t,
    oldset: *mut sigset_t,
) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let checked_oldset;
        let checked_set;
        {
            let v = scheduler
                .current_task_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            checked_set = if set.is_null() {
                None
            } else {
                v.check_user_ptr::<u32>(set)?;
                Some(&*set)
            };
            checked_oldset = if oldset.is_null() {
                None
            } else {
                v.check_user_ptr::<u32>(oldset)?;
                Some(&mut *oldset)
            };
        }
        scheduler
            .current_task_mut()
            .signal
            .change_signal_mask(how, checked_set, checked_oldset)
    })
}
