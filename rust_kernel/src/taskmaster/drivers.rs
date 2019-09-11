//! This file contains drivers definitions

use super::SysResult;

use super::fd_interface::Mode;
use super::vfs::{InodeId, VFS};
use super::IpcResult;

pub mod ipc;
pub use ipc::Fifo;
pub use ipc::Pipe;
pub use ipc::Socket;

pub mod tty;
pub use tty::TtyDevice;

pub mod file;
pub use file::{Ext2DriverFile, Ext2FileOperation};

pub mod disk;
pub use disk::{BiosInt13hInstance, DiskDriver, DiskFileOperation, DiskWrapper, IdeAtaInstance};
// pub use disk::DiskDriver;

use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::{off_t, stat, statfs, termios, Errno, Pid, Whence};
use sync::dead_mutex::DeadMutex;

/// This Trait represent a File Descriptor in Kernel
/// It cas be shared between process (cf Fork()) and for two user fd (cf Pipe()) or one (cf Socket() or Fifo())
pub trait FileOperation: core::fmt::Debug + Send {
    /// Invoqued when a new FD is registered
    fn register(&mut self, _access_mode: Mode) {}

    /// Invoqued quen a FD is droped
    fn unregister(&mut self, _access_mode: Mode) {}

    fn lseek(&mut self, _offset: off_t, _whence: Whence) -> SysResult<off_t> {
        Err(Errno::EINVAL)
    }

    /// Read something from the File Descriptor: Important ! When in blocked syscall, the slice must be verified before read op
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>>;

    /// Write something into the File Descriptor: Important ! When in blocked syscall, the slice must be verified before write op
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>>;

    fn fstat(&mut self, _stat: &mut stat) -> SysResult<u32> {
        log::error!("UNIMPLEMENTED: fstat called on a non ext2 file");
        Ok(0)
    }

    fn tcgetattr(&self, _termios_p: &mut termios) -> SysResult<u32> {
        Err(Errno::ENOTTY)
    }

    fn tcsetattr(&mut self, _optional_actions: u32, _termios_p: &termios) -> SysResult<u32> {
        Err(Errno::ENOTTY)
    }

    fn tcgetpgrp(&self) -> SysResult<Pid> {
        Err(Errno::ENOTTY)
    }

    fn tcsetpgrp(&mut self, _pgid_id: Pid) -> SysResult<u32> {
        Err(Errno::ENOTTY)
    }

    fn isatty(&mut self) -> SysResult<u32> {
        Err(Errno::ENOTTY)
    }

    fn fstatfs(&mut self, _buf: &mut statfs) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }
}

#[derive(Debug)]
pub struct DefaultFileOperation;

impl FileOperation for DefaultFileOperation {
    fn register(&mut self, _access_mode: Mode) {}
    fn unregister(&mut self, _access_mode: Mode) {}
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::EINVAL)
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::EINVAL)
    }
}

/// This Trait represent a File Driver in the VFS
pub trait Driver: core::fmt::Debug + Send {
    /// Open method of a file
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>>;
}

#[derive(Debug)]
pub struct DefaultDriver;

impl Driver for DefaultDriver {
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(DefaultFileOperation))?;
        Ok(IpcResult::Done(res))
    }
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
