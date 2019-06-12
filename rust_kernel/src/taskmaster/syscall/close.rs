//! Close a file descriptor

use super::SysResult;

pub fn sys_close(_fd: i32) -> SysResult<u32> {
    Ok(0)
}
