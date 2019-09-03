use super::SysResult;

use super::scheduler::SCHEDULER;
use super::Fd;

use libc_binding::stat;

pub fn sys_fstat(fd: Fd, buf: *mut stat) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_ref_mut::<stat>(buf)?
        };
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &mut fd_interface.get_file_operation(fd)?;
        file_operation.fstat(safe_buf)
    })
}
