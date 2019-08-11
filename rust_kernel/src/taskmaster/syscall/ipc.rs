//! This is the root file of all the IPC syscalls

use super::SysResult;

use super::scheduler;

mod write;
pub use write::sys_write;

mod socket;
pub use socket::{sys_socketcall, SocketArgsPtr};

mod pipe;
pub use pipe::sys_pipe;

mod dup;
pub use dup::sys_dup;

mod dup2;
pub use dup2::sys_dup2;
