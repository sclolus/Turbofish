use super::clone::sys_clone;
use super::SysResult;
use core::ffi::c_void;

/// Fork a process
pub fn sys_fork(kernel_esp: u32) -> SysResult<u32> {
    sys_clone(
        kernel_esp,
        0 as *const c_void,
        0, /*CLONE_CHILD_CLEARTID|CLONE_CHILD_SETTID|SIGCHLD*/
    )
}
