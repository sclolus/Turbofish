use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::{c_char, mode_t};

pub fn sys_mkdir(path: *const c_char, _mode: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let _safe_path = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(path)?
        };
        unimplemented!()
    })
}
