use super::scheduler::SCHEDULER;
use super::SysResult;

/// The isatty() function shall test whether fildes, an open file
/// descriptor, is associated with a terminal device.
/// The isatty() function shall return 1 if fildes is associated with
/// a terminal; otherwise, it shall return 0 and may set errno to
/// indicate the error.
/// The isatty() function may fail if:
/// [EBADF]
///     The fildes argument is not a valid open file descriptor.
/// [ENOTTY]
///     The file associated with the fildes argument is not a
///     terminal.
pub fn sys_isatty(fildes: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &mut fd_interface.get_file_operation(fildes)?;
        file_operation.isatty()
    })
}
