use super::SysResult;

use libc_binding::{c_char, mode_t};

pub fn sys_mkdir(_path: *const c_char, _mode: mode_t) -> SysResult<u32> {
    unimplemented!()
}
