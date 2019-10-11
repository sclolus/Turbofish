//! sys_get_kernel_properties()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::GLOBAL_TIME;

use libc_binding::kernel;

/// Get some informations from kernel
pub fn sys_get_kernel_properties(kernel: *mut kernel) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let safe_kernel = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();
            v.make_checked_ref_mut(kernel)?
        };
        safe_kernel.cpu_frequency =
            unsafe { GLOBAL_TIME.as_ref().expect("Woot ?").cpu_frequency() };
        Ok(0)
    })
}
