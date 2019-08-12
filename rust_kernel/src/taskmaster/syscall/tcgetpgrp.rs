use super::SysResult;
use crate::terminal::TERMINAL;

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
// TODO: file descriptor argument
pub fn sys_tcgetpgrp(_fildes: i32) -> SysResult<u32> {
    unpreemptible_context!({
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                //TODO: change this 1
                .get_line_discipline(1)
                .tcgetpgrp()
        }
    });
    Ok(0)
}
