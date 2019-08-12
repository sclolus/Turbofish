use super::SysResult;
use crate::terminal::TERMINAL;
use libc_binding::termios;
use super::scheduler::SCHEDULER;

/// The tcgetattr() function shall get the parameters associated with
/// the terminal referred to by fildes and store them in the termios
/// structure referenced by termios_p. The fildes argument is an open
/// file descriptor associated with a terminal.
///
/// The termios_p argument is a pointer to a termios structure.
///
/// The tcgetattr() operation is allowed from any process.
// TODO: file descriptor argument
pub fn sys_tcgetattr(_fildes: i32, termios_p: *mut termios) -> SysResult<u32> {
    unpreemptible_context!({
        {
            let scheduler = SCHEDULER.lock();
            let v = scheduler
                .current_task()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr(termios_p)?;
        }
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                // TODO: change this 1
                .get_line_discipline(1)
                .tcgetattr(&mut *termios_p);
        }
    });
    Ok(0)
}
