use super::SysResult;
use crate::terminal::TERMINAL;
use libc_binding::termios;

pub fn sys_tcsetattr(
    fildes: i32,
    optional_actions: u32,
    termios_p: *const termios,
) -> SysResult<u32> {
    unpreemptible_context!({
        // TODO: change this 1
        // TODO: check termios_p pointer
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(1)
                .tcsetattr(optional_actions, &*termios_p);
        }
    });
    Ok(0)
}
