use super::SysResult;

use libc_binding::c_char;

pub fn sys_link(_path1: *const c_char, _path2: *const c_char) -> SysResult<u32> {
    unimplemented!()
}
