use super::scheduler::SCHEDULER;
use super::SysResult;

use libc_binding::{c_char, gid_t, uid_t};

pub fn sys_chown(path: *const c_char, _owner: uid_t, _group: gid_t) -> SysResult<u32> {
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
