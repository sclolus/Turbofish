use super::Pid;
use super::SysResult;
use crate::terminal::TERMINAL;

pub fn sys_tcsetpgrp(fildes: i32, pgid_id: Pid) -> SysResult<u32> {
    unpreemptible_context!({
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(1)
                .tcsetpgrp(pgid_id);
        }
    });
    Ok(0)
}
