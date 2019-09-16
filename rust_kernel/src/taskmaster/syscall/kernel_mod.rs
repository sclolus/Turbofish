//! sys_rmmod() && sys_insmod

use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::c_char;

/// Insert a kernel module
pub fn sys_insmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let _safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        Ok(0)
    })
}

/// Remove a kernel module
pub fn sys_rmmod(modname: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let _safe_modname = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(modname)?
        };
        Ok(0)
    })
}
