//! tcgetattr syscall
use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;
use libc_binding::termios;

/// The tcgetattr() function shall get the parameters associated with
/// the terminal referred to by fildes and store them in the termios
/// structure referenced by termios_p. The fildes argument is an open
/// file descriptor associated with a terminal.
///
/// The termios_p argument is a pointer to a termios structure.
///
/// The tcgetattr() operation is allowed from any process.
/// [EBADF]
///     The fildes argument is not a valid file descriptor.
/// [ENOTTY]
///     The file associated with fildes is not a terminal.
pub fn sys_tcgetattr(fildes: Fd, termios_p: *mut termios) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();
        let termios_p = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.make_checked_ref_mut(termios_p)?
        };
        let fd_interface = &scheduler
            .current_thread_group_running()
            .file_descriptor_interface;

        let file_operation = &fd_interface.get_file_operation(fildes)?;
        file_operation.tcgetattr(termios_p)
    })
}
