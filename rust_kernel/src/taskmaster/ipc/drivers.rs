//! This file contains drivers definitions

use super::SysResult;

use super::IpcResult;
use super::Mode;

pub mod tty;
pub use tty::TtyDevice;

pub mod pipe;
pub use pipe::Pipe;

pub mod fifo;
pub use fifo::Fifo;

pub mod socket;
pub use socket::Socket;

use alloc::sync::Arc;
use sync::dead_mutex::DeadMutex;

/// This Trait represent a File Descriptor in Kernel
/// It cas be shared between process (cf Fork()) and for two user fd (cf Pipe()) or one (cf Socket() or Fifo())
pub trait FileOperation: core::fmt::Debug + Send {
    /// Invoqued when a new FD is registered
    fn register(&mut self, access_mode: Mode);
    /// Invoqued quen a FD is droped
    fn unregister(&mut self, access_mode: Mode);
    /// Read something from the File Descriptor: Important ! When in blocked syscall, the slice must be verified before read op
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>>;
    /// Write something into the File Descriptor: Important ! When in blocked syscall, the slice must be verified before write op
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>>;
}

/// This Trait represent a File Driver in the VFS
pub trait Driver: core::fmt::Debug + Send {
    /// Open method of a file
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>>;
    /// Get a reference to the inode
    fn set_inode_id(&mut self, inode_id: usize);
}

/// Get an universal file operation identifiant
pub fn get_file_op_uid() -> usize {
    unsafe {
        FILE_OP_UID += 1;
        FILE_OP_UID
    }
}

static mut FILE_OP_UID: usize = 0;

// /// Here the type of the Kernel File Descriptor
// #[derive(Clone, Copy, Debug, Eq, PartialEq, TryClone)]
// enum FileOperationType {
//     Pipe,
//     Fifo,
//     Socket,
//     Stdin,
//     Stdout,
//     Stderr,
// }
