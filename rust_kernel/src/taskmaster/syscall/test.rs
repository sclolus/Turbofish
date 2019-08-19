use super::SysResult;
use libc_binding::Errno;

extern "C" {
    fn _sys_test() -> i32;
}

/// Preemptif coherency checker
pub unsafe fn sys_test() -> SysResult<u32> {
    if _sys_test() == 0 {
        Ok(0)
    } else {
        Err(Errno::EPERM)
    }
}
