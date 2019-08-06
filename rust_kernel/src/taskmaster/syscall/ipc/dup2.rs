//! This file contains the description of the dup2 syscall

use super::SysResult;

// use errno::Errno;

/// Duplicate a file descriptor
pub fn sys_dup2(_old_fd: u32, _new_fd: u32) -> SysResult<u32> {
    unpreemptible_context!({ Ok(0) })
}
