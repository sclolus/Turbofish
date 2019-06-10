//! Close a file descriptor

use super::SysResult;

pub fn sys_close(_fd: u32) -> SysResult<u32> {
    Ok(0)
}
