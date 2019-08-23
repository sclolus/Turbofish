//! tcgetpgrp syscall
use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;

/// The tcgetpgrp() function shall return the value of the process
/// group ID of the foreground process group associated with the
/// terminal.
///
/// If there is no foreground process group, tcgetpgrp() shall return
/// a value greater than 1 that does not match the process group ID of
/// any existing process group.
///
/// The tcgetpgrp() function is allowed from a process that is a
/// member of a background process group; however, the information may
/// be subsequently changed by a process that is a member of a
/// foreground process group.
/// [EBADF]
///     The fildes argument is not a valid file descriptor.
/// [EINVAL]
///     This implementation does not support the value in the pgid_id
///     argument.
/// [EIO]
///     The process group of the writing process is orphaned, the
///     calling thread is not blocking SIGTTOU, and the process is not
///     ignoring SIGTTOU.
/// [ENOTTY]
///     The calling process does not have a controlling terminal, or
///     the file is not the controlling terminal, or the controlling
///     terminal is no longer associated with the session of the
///     calling process.
/// [EPERM]
///     The value of pgid_id is a value supported by the
///     implementation, but does not match the process group ID of a
///     process in the same session as the calling process.
pub fn sys_tcgetpgrp(fildes: Fd) -> SysResult<u32> {
    unpreemptible_context!({
        dbg!("tcgetpgrp");
        let scheduler = SCHEDULER.lock();
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &fd_interface.get_file_operation(fildes)?;
        Ok(file_operation.tcgetpgrp()? as u32)
    })
}
