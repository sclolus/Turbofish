//! This file contains drivers definitions

use super::SysResult;

use super::vfs;
use super::vfs::Path;
use super::vfs::{InodeId, VFS};
use super::Credentials;
use super::IpcResult;

pub mod ipc;
pub use ipc::{FifoDriver, FifoFileOperation, Pipe, Socket};

pub mod tty;
pub use tty::TtyDevice;

pub mod file;
pub use file::{Ext2DriverFile, Ext2FileOperation};

pub mod disk;
pub use disk::{BiosInt13hInstance, DiskDriver, DiskFileOperation, DiskWrapper, IdeAtaInstance};
// pub use disk::DiskDriver;

use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::{
    gid_t, off_t, stat, statfs, termios, uid_t, Errno, FileType, OpenFlags, Pid, Whence,
};
use sync::dead_mutex::DeadMutex;

/// This Trait represent a File Descriptor in Kernel
/// It cas be shared between process (cf Fork()) and for two user fd (cf Pipe()) or one (cf Socket() or Fifo())
pub trait FileOperation: core::fmt::Debug + Send {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Err(Errno::ENOSYS)
    }
    /// Invoqued when a new FD is registered
    fn register(&mut self, _flags: OpenFlags) {}

    /// Invoqued quen a FD is droped
    fn unregister(&mut self, _flags: OpenFlags) {}
    fn lseek(&mut self, _offset: off_t, _whence: Whence) -> SysResult<off_t> {
        Err(Errno::EINVAL)
    }

    /// Read something from the File Descriptor: Important ! When in blocked syscall, the slice must be verified before read op
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::ENOSYS)
    }

    /// Write something into the File Descriptor: Important ! When in blocked syscall, the slice must be verified before write op
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::ENOSYS)
    }

    fn fstat(&mut self, stat: &mut stat) -> SysResult<u32> {
        let inode_id = self.get_inode_id()?;
        VFS.lock()
            .get_inode(inode_id)
            .expect("no such inode")
            .stat(stat)?;
        Ok(0)
    }

    fn fchmod(&mut self, _mode: FileType) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }

    fn fchown(&mut self, _owner: uid_t, _group: gid_t) -> SysResult<u32> {
        Err(Errno::ENOSYS)
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

    fn bind(&mut self, _cwd: &Path, _creds: &Credentials, _sockaddr: Path) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }

    fn send_to(&mut self, _buf: &[u8], _flags: u32, _sockaddr_opt: Option<Path>) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }

    fn recv_from(
        &mut self,
        _buf: &mut [u8],
        _flags: u32,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        Err(Errno::ENOSYS)
    }
}

#[derive(Debug)]
pub struct DefaultFileOperation;

impl FileOperation for DefaultFileOperation {
    fn register(&mut self, _flags: OpenFlags) {}
    fn unregister(&mut self, _flags: OpenFlags) {}
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
    fn open(&mut self, flags: OpenFlags)
        -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>>;
    fn send_from(&mut self, _buf: &[u8], _flags: u32, _sender: Option<Path>) -> SysResult<u32> {
        Err(Errno::ENOSYS)
    }
    fn recv_from(
        &mut self,
        _buf: &mut [u8],
        _flags: u32,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        Err(Errno::ENOSYS)
    }
}

#[derive(Debug)]
pub struct DefaultDriver;

impl Driver for DefaultDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
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
