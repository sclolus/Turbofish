use super::SysResult;

use super::scheduler::SCHEDULER;
use super::Fd;

use libc_binding::stat;

pub fn sys_fstat(_fd: Fd, buf: *mut stat) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let _safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_ref_mut::<stat>(buf)?
        };
        // TODO: Use VFS stat on FD (send stat structure)
        // TODO: Return
        Ok(0)
    })
}
