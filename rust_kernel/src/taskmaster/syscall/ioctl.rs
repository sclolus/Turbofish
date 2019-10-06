use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;

use core::convert::TryFrom;
use libc_binding::IoctlCmd;

pub fn sys_ioctl(fildes: Fd, cmd: u32, arg: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let cmd = IoctlCmd::try_from(cmd)?;
        let scheduler = SCHEDULER.lock();

        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &mut fd_interface.get_file_operation(fildes)?;
        Ok(file_operation.ioctl(&scheduler, cmd, arg)? as u32)
    })
}
