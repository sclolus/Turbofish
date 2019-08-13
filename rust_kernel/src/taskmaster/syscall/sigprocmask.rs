//! This file contain all the signal related syscall code

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::signal_interface::sigset_t;

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
