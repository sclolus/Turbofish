use super::get_file_op_uid;
use super::Credentials;
use super::InodeId;
use super::IpcResult;
use super::Path;
use super::SysResult;
use super::VFS;
use super::{Driver, FileOperation};

use super::vfs;

pub mod pipe;
pub use pipe::Pipe;

pub mod fifo;
pub use fifo::{FifoDriver, FifoFileOperation};

pub mod socket;
pub use socket::{ConnectedSocket, SocketDgram, SocketDriver};

pub struct Buf([u8; Self::BUF_SIZE]);

/// Deref boilerplate for Buf
impl core::ops::Deref for Buf {
    type Target = [u8; Self::BUF_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// DerefMut boilerplate for Buf
impl core::ops::DerefMut for Buf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Default boilerplate for Buf
impl Default for Buf {
    fn default() -> Self {
        Self {
            0: [0; Self::BUF_SIZE],
        }
    }
}

/// Debug boilerplate for Buf
impl core::fmt::Debug for Buf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Hidden Buf content")
    }
}

/// Buf implementation
impl Buf {
    pub const BUF_SIZE: usize = 128;
}
