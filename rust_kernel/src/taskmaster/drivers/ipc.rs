use super::get_file_op_uid;
use super::IpcResult;
use super::SysResult;
use super::VFS;
use super::{Driver, FileOperation};

use super::vfs;

pub mod pipe;
pub use pipe::Pipe;

pub mod fifo;
pub use fifo::{FifoDriver, FifoFileOperation};

pub mod socket;
pub use socket::Socket;
