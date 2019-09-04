use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::c_char;

pub fn sys_rename(old: *const c_char, new: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let (_safe_old, _safe_new) = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            (v.make_checked_str(old)?, v.make_checked_str(new)?)
        };
        unimplemented!()
    })
}
