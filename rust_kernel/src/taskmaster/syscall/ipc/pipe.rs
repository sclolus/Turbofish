//! This file contains the description of the pipe syscall

use super::SysResult;

// use errno::Errno;

/// Create pipe
pub fn sys_pipe(_in_fd: u32, _out_fd: u32) -> SysResult<u32> {
    unpreemptible_context!({ Ok(0) })
}
