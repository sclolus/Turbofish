use super::SysResult;

use libc_binding::c_char;

pub fn sys_rmdir(_path: *const c_char) -> SysResult<u32> {
    unimplemented!()
}
