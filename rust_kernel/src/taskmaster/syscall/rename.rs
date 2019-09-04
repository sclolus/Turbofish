use super::SysResult;

use libc_binding::c_char;

pub fn sys_rename(_old: *const c_char, _new: *const c_char) -> SysResult<u32> {
    unimplemented!()
}
