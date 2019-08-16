use super::getpgid::sys_getpgid;
use super::SysResult;

pub fn sys_getpgrp() -> SysResult<u32> {
    sys_getpgid(0)
}
