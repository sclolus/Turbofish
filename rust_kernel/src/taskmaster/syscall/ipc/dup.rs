//! This file contains the description of the dup syscall

use super::SysResult;

// use errno::Errno;

/// Duplicate a file descriptor
pub fn sys_dup(_old_fd: u32) -> SysResult<u32> {
    unpreemptible_context!({ Ok(0) })
}
