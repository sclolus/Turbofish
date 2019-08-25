//! This file contain all the signal related syscall code

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::signal_interface::StructSigaction;

use core::convert::TryInto;
use libc_binding::Errno;

/// Register a new handler for a specified signum with sigaction params
pub fn sys_sigaction(
    signum: u32,
    act: *const StructSigaction,
    old_act: *mut StructSigaction,
) -> SysResult<u32> {
    unpreemptible_context!({
        let checked_act;
        let checked_old_act;
        let mut scheduler = SCHEDULER.lock();
        {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            checked_act = if act.is_null() {
                None
            } else {
                Some(v.make_checked_ref(act)?)
            };
            checked_old_act = if old_act.is_null() {
                None
            } else {
                Some(v.make_checked_ref_mut(old_act)?)
            }
        }
        // TODO: Use old_act
        let old = scheduler
            .current_thread_mut()
            .signal
            .new_handler(signum.try_into().map_err(|_| Errno::EINVAL)?, checked_act)?;
        if let Some(old_act) = checked_old_act {
            *old_act = old;
        }
        Ok(0)
    })
}
