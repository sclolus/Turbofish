use super::SysResult;
use crate::terminal::TERMINAL;

pub fn sys_tcgetpgrp(fildes: i32) -> SysResult<u32> {
    unpreemptible_context!({
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(1)
                .tcgetpgrp()
        }
    });
    Ok(0)
}
