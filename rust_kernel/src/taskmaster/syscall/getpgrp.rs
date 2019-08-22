use super::getpgid::sys_getpgid;
use super::SysResult;

/// The getpgrp() function shall return the process group ID of the
/// calling process.
pub fn sys_getpgrp() -> SysResult<u32> {
    sys_getpgid(0)
}
