use super::SysResult;

use super::scheduler::SCHEDULER;
use super::Fd;

use libc_binding::statfs;

/// The statfs() system call returns information about a mounted filesystem.
/// `path` is the pathname of any file within the mounted filesystem.
/// `buf` is a  pointer  to  a  statfs structure.
///
/// fstatfs()  returns  the  same information about an
/// open file referenced by descriptor fd.
pub fn sys_fstatfs(fd: Fd, buf: *mut statfs) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_ref_mut(buf)?
        };
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &mut fd_interface.get_file_operation(fd)?;
        file_operation.fstatfs(safe_buf)
    })
}
