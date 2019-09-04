use super::SysResult;

use libc_binding::c_char;

pub fn sys_access(_path: *const c_char, _amode: i32) -> SysResult<u32> {
    unimplemented!()
}
