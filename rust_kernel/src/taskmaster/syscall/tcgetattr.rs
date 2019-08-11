use super::SysResult;
use crate::terminal::TERMINAL;
use libc_binding::termios;

/// The tcgetattr() function shall get the parameters associated with
/// the terminal referred to by fildes and store them in the termios
/// structure referenced by termios_p. The fildes argument is an open
/// file descriptor associated with a terminal.
///
/// The termios_p argument is a pointer to a termios structure.
///
/// The tcgetattr() operation is allowed from any process.
pub fn sys_tcgetattr(_fildes: i32, termios_p: *mut termios) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: change this 1
        // TODO: check termios_p pointer
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(1)
                .tcgetattr(&mut *termios_p);
        }
    });
    Ok(0)
}
