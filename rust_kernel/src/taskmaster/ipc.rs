//! This file contains all the stuff about File Descriptors and generals IPC

use super::SysResult;

use libc_binding::Errno;

use alloc::sync::Arc;

use fallible_collections::btree::BTreeMap;
use fallible_collections::FallibleArc;
use fallible_collections::TryClone;

use try_clone_derive::TryClone;

use sync::DeadMutex;

pub type Fd = u32;

mod fifo;
use fifo::Fifo;
mod pipe;
use pipe::Pipe;
mod socket;
use socket::Socket;

pub mod std;
pub use std::Std;

use self::std::{Stderr, Stdin, Stdout};

/// Dependance du Vfs
use super::dummy_vfs::{DummyVfs, DUMMY_VFS};

/// The User File Descriptor are sorted into a Binary Tree
/// Key is the user number and value the structure FileDescriptor
#[derive(Debug, TryClone)]
pub struct FileDescriptorInterface {
    user_fd_list: BTreeMap<Fd, FileDescriptor>,
}

/// Describe what to do after an IPC request and result return
#[derive(Debug)]
pub enum IpcResult<T> {
    /// Can continue thread execution normally
    Done(T),
    /// the user should wait for his IPC request
    Wait(T),
}

/// The Access Mode of the File Descriptor
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryClone)]
pub enum Mode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

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
    fn open(&mut self) -> Arc<DeadMutex<dyn FileOperation>>;
    fn set_inode_id(&mut self, inode_id: usize);
}

/// Here the type of the Kernel File Descriptor
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryClone)]
enum FileOperationType {
    Pipe,
    Fifo,
    Socket,
    Stdin,
    Stdout,
    Stderr,
}

/// This structure design a User File Descriptor
/// We can normally clone the Arc
#[derive(Debug, TryClone)]
struct FileDescriptor {
    access_mode: Mode,
    fd_type: FileOperationType,
    kernel_fd: Arc<DeadMutex<dyn FileOperation>>,
}

/// Standard implementation of an user File Descriptor
impl FileDescriptor {
    /// When a new FileDescriptor is invoqued, Increment reference
    fn new(
        access_mode: Mode,
        fd_type: FileOperationType,
        kernel_fd: Arc<DeadMutex<dyn FileOperation>>,
    ) -> Self {
        kernel_fd.lock().register(access_mode);
        Self {
            access_mode,
            fd_type,
            kernel_fd,
        }
    }
}

/// Drop boilerplate for an FileDescriptor structure. Decremente reference
impl Drop for FileDescriptor {
    fn drop(&mut self) {
        self.kernel_fd.lock().unregister(self.access_mode);
    }
}

/// Main implementation
impl FileDescriptorInterface {
    const MAX_FD: Fd = 128;

    /// Global constructor
    pub fn new() -> Self {
        let mut r = Self {
            // New BTreeMap does not allocate memory
            user_fd_list: BTreeMap::new(),
        };
        r.open_std(1)
            .expect("Global constructor of STD devices fail");
        r
    }

    /// Open Stdin, Stdout and Stderr
    /// The File Descriptors between 0..2 are automaticely closed
    pub fn open_std(&mut self, controlling_terminal: usize) -> SysResult<()> {
        let _r = self.close_fd(0);
        let _r = self.close_fd(1);
        let _r = self.close_fd(2);
        let stdin = Arc::try_new(DeadMutex::new(Stdin::new(controlling_terminal)))?;
        let stdout = Arc::try_new(DeadMutex::new(Stdout::new(controlling_terminal)))?;
        let stderr = Arc::try_new(DeadMutex::new(Stderr::new(controlling_terminal)))?;

        let _fd = self.user_fd_list.try_insert(
            0,
            FileDescriptor::new(Mode::ReadOnly, FileOperationType::Stdin, stdin),
        )?;
        let _fd = self.user_fd_list.try_insert(
            1,
            FileDescriptor::new(Mode::WriteOnly, FileOperationType::Stdout, stdout),
        )?;
        let _fd = self.user_fd_list.try_insert(
            2,
            FileDescriptor::new(Mode::WriteOnly, FileOperationType::Stderr, stderr),
        )?;
        Ok(())
    }

    /// Made two File Descriptors connected with a Pipe
    pub fn new_pipe(&mut self) -> SysResult<(Fd, Fd)> {
        let pipe = Arc::try_new(DeadMutex::new(Pipe::new()))?;
        let cloned_pipe = pipe.clone();

        let input_fd = self.insert_user_fd(Mode::ReadOnly, FileOperationType::Pipe, pipe)?;
        let output_fd = self
            .insert_user_fd(Mode::ReadOnly, FileOperationType::Pipe, cloned_pipe)
            .map_err(|e| {
                let _r = self.user_fd_list.remove(&input_fd);
                e
            })?;

        Ok((input_fd, output_fd))
    }

    /// Open a Fifo. Block until the fifo is not open in two directions.
    #[allow(dead_code)]
    pub fn open_fifo(&mut self, access_mode: Mode) -> SysResult<IpcResult<Fd>> {
        if access_mode == Mode::ReadWrite {
            return Err(Errno::EACCES);
        }

        let fifo = Arc::try_new(DeadMutex::new(Fifo::new()))?;
        let fd = self.insert_user_fd(access_mode, FileOperationType::Fifo, fifo)?;

        Ok(IpcResult::Done(fd))
    }

    /// Open a Socket
    /// The socket type must be pass as parameter
    #[allow(dead_code)]
    pub fn open_socket(&mut self, access_mode: Mode) -> SysResult<Fd> {
        let socket = Arc::try_new(DeadMutex::new(Socket::new()))?;
        let fd = self.insert_user_fd(access_mode, FileOperationType::Socket, socket)?;

        Ok(fd)
    }

    /// Read something from the File Descriptor: Can block
    /// Important ! When in blocked syscall, the slice must be verified before read op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    pub fn read(&mut self, fd: Fd, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::EBADF)?;

        elem.kernel_fd.lock().read(buf)
    }

    /// Write something into the File Descriptor: Can block
    /// Important ! When in blocked syscall, the slice must be verified before write op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    pub fn write(&mut self, fd: Fd, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::EBADF)?;

        elem.kernel_fd.lock().write(buf)
    }

    /// Duplicate one File Descriptor
    pub fn dup(&mut self, oldfd: Fd) -> SysResult<Fd> {
        for (key, elem) in &self.user_fd_list {
            if *key == oldfd {
                let new_elem = elem.try_clone()?;
                let newfd = self.get_lower_fd_value().ok_or::<Errno>(Errno::EMFILE)?;

                self.user_fd_list.try_insert(newfd, new_elem)?;
                return Ok(newfd);
            }
        }
        Err(Errno::EBADF)
    }

    /// Duplicate one file descriptor with possible override
    pub fn dup2(&mut self, oldfd: Fd, newfd: Fd) -> SysResult<Fd> {
        if newfd > Self::MAX_FD {
            return Err(Errno::EBADF);
        }

        // If oldfd is not a valid file descriptor, then the call fails, and newfd is not closed.
        for (key, elem) in &self.user_fd_list {
            if *key == oldfd {
                let new_elem = elem.try_clone()?;
                let _r = self.close_fd(newfd);

                self.user_fd_list.try_insert(newfd, new_elem)?;
                return Ok(newfd);
            }
        }
        Err(Errno::EBADF)
    }

    /// Clone one file descriptor
    pub fn close_fd(&mut self, fd: Fd) -> SysResult<()> {
        self.user_fd_list.remove(&fd).ok_or::<Errno>(Errno::EBADF)?;
        Ok(())
    }

    /// Insert a new User File Descriptor atached to a Kernel File Descriptor:
    /// return value: User File Descriptor index
    fn insert_user_fd(
        &mut self,
        mode: Mode,
        fd_type: FileOperationType,
        kernel_fd: Arc<DeadMutex<dyn FileOperation>>,
    ) -> SysResult<Fd> {
        let user_fd = self.get_lower_fd_value().ok_or::<Errno>(Errno::EMFILE)?;
        self.user_fd_list
            .try_insert(user_fd, FileDescriptor::new(mode, fd_type, kernel_fd))?;
        Ok(user_fd)
    }

    /// Get the first available File Descriptor number
    fn get_lower_fd_value(&self) -> Option<Fd> {
        let mut lower_fd = 0;

        for (key, _) in &self.user_fd_list {
            if lower_fd < *key {
                return Some(lower_fd);
            } else {
                lower_fd += 1;
            }
        }
        if lower_fd > Self::MAX_FD {
            None
        } else {
            Some(lower_fd)
        }
    }
}

/// Some boilerplate to check if all is okay
impl Drop for FileDescriptorInterface {
    fn drop(&mut self) {
        println!("FD interface droped");
    }
}
