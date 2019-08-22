//! This file contain all the signal related syscall code

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::signal_interface::StructSigaction;

use core::convert::TryInto;
use libc_binding::Errno;

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
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr::<StructSigaction>(act)?;
            if old_act as usize != 0 {
                v.check_user_ptr::<StructSigaction>(old_act)?;
            }
        }
        // TODO: Use old_act
        *old_act = scheduler.current_thread_mut().signal.new_handler(
            signum.try_into().map_err(|_| Errno::EINVAL)?,
            act.as_ref().expect("Null PTR"),
        )?;
        Ok(0)
    })
}
