use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::c_char;

pub fn sys_is_str_valid(filename: *const c_char) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let _safe_filename = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();
            v.make_checked_str(filename)?;
        };
        Ok(0)
    })
}
