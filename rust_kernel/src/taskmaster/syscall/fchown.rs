use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;

use libc_binding::{gid_t, uid_t};

pub fn sys_fchown(fd: Fd, owner: uid_t, group: gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        let tg = scheduler.current_thread_group();

        let creds = &tg.credentials;
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &mut fd_interface.get_file_operation(fd)?;
        file_operation.fchown(creds, owner, group)
    })
}
