//! Delete a name and possibly the file it refers to

use super::SysResult;

pub fn sys_unlink(_pathname: *const u8) -> SysResult<u32> {
    Ok(0)
}
