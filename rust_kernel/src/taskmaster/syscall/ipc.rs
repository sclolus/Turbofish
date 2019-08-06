//! This is the root file of all the IPC syscalls

use super::SysResult;

use super::scheduler;

mod socket;
pub use socket::{sys_socketcall, SocketArgsPtr};

mod pipe;
pub use pipe::sys_pipe;
