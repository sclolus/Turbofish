use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::c_char;

pub fn sys_rmdir(path: *const c_char) -> SysResult<u32> {
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
