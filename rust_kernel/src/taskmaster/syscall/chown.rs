use super::SysResult;

use libc_binding::{c_char, gid_t, uid_t};

pub fn sys_chown(_path: *const c_char, _owner: uid_t, _group: gid_t) -> SysResult<u32> {
    unimplemented!()
}
