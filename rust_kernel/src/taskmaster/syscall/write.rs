use super::SysResult;
use libc_binding::Errno;

/// Write something into the screen
pub fn sys_write(fd: i32, buf: *const u8, count: usize) -> SysResult<u32> {
    if fd != 1 && fd != 2 {
        Err(Errno::EBADF)
    } else {
        unsafe {
            unpreemptible_context!({
                if fd == 2 {
                    eprint!(
                        "{}",
                        core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count))
                    );
                } else {
                    print!(
                        "{}",
                        core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count))
                    );
                }
            })
        }
        Ok(count as u32)
    }
}
