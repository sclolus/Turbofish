use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;
use core::convert::TryFrom;

use libc_binding::{mode_t, FileType};

pub fn sys_fchmod(fd: Fd, mode: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        let tg = scheduler.current_thread_group();

        let creds = &tg.credentials;
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let mode = FileType::try_from(mode)?;

        // this extract only permission and special bits from mode
        let pure_mode = FileType::extract_pure_mode(mode);

        let file_operation = &mut fd_interface.get_file_operation(fd)?;
        file_operation.fchmod(creds, pure_mode)
    })
}
