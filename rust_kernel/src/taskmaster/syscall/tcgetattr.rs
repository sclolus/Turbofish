use super::SysResult;
use crate::terminal::TERMINAL;
use libc_binding::termios;

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
