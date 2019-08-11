//! This file contains all the stuff about File Descriptors and generals IPC

use super::SysResult;

use errno::Errno;

use alloc::sync::Arc;

use fallible_collections::TryClone;

use fallible_collections::btree::BTreeMap;
use fallible_collections::FallibleArc;

use sync::DeadMutex;

pub type Fd = u32;

mod fifo;
use fifo::Fifo;
mod pipe;
use pipe::Pipe;
mod socket;
use socket::Socket;

/// The User File Descriptor are sorted into a Binary Tree
/// Key is the user number and value the structure UserFileDescriptor
#[derive(Debug)]
pub struct FileDescriptorInterface {
    user_fd_list: BTreeMap<Fd, UserFileDescriptor>,
}

/// The Access Mode of the File Descriptor
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// This Trait represent a File Descriptor in Kernel
/// It cas be shared between process (cf Fork()) and for two user fd (cf Pipe())
trait KernelFileDescriptor: core::fmt::Debug + Send {
    /// Invoqued when a new FD is registered
    fn register(&mut self, access_mode: Mode);
    /// Invoqued quen a FD is droped
    fn unregister(&mut self, access_mode: Mode);
    /// Read something from the File Descriptor: Important ! When in blocked syscall, the slice must be verified before read op
    fn read(&mut self, buf: &mut [u8]) -> SysResult<i32>;
    /// Write something into the File Descriptor: Important ! When in blocked syscall, the slice must be verified before write op
    fn write(&mut self, buf: &[u8]) -> SysResult<i32>;
}

/// Here the type of the Kernel File Descriptor
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KernelFileDescriptorType {
    Pipe,
    Fifo,
    Socket,
}

/// This structure design a User File Descriptor
/// We can normally clone the Arc
#[derive(Debug)]
struct UserFileDescriptor {
    access_mode: Mode,
    fd_type: KernelFileDescriptorType,
    kernel: Arc<DeadMutex<dyn KernelFileDescriptor>>,
}

/// TryClone boilerplate for UserFileDescriptor: Contains exception for Arc
impl TryClone for UserFileDescriptor {
    fn try_clone(&self) -> Result<Self, alloc::collections::CollectionAllocErr> {
        Ok(Self {
            access_mode: self.access_mode,
            fd_type: self.fd_type,
            // Cloning a Arc does not allocate memory. Just increments the ref count
            kernel: self.kernel.clone(),
        })
    }
}

/// Standard implementation of an user File Descriptor
impl UserFileDescriptor {
    /// When a new UserFileDescriptor is invoqued, Increment reference
    fn new(
        access_mode: Mode,
        fd_type: KernelFileDescriptorType,
        kernel: Arc<DeadMutex<dyn KernelFileDescriptor>>,
    ) -> Self {
        kernel.lock().register(access_mode);
        Self {
            access_mode,
            fd_type,
            kernel,
        }
    }
}

/// Drop boilerplate for an UserFileDescriptor structure. Decremente reference
impl Drop for UserFileDescriptor {
    fn drop(&mut self) {
        self.kernel.lock().unregister(self.access_mode);
    }
}

/// Main implementation
impl FileDescriptorInterface {
    const MAX_FD: Fd = 128;

    /// Global constructor
    pub fn new() -> Self {
        Self {
            // New BTreeMap does not allocate memory
            user_fd_list: BTreeMap::new(),
        }
    }

    /// Made two File Descriptors connected with a Pipe
    pub fn new_pipe(&mut self) -> SysResult<(Fd, Fd)> {
        let pipe = Arc::try_new(DeadMutex::new(Pipe::new()))?;
        let cloned_pipe = pipe.clone();

        let input_fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(
                        fd,
                        UserFileDescriptor::new(
                            Mode::ReadOnly,
                            KernelFileDescriptorType::Pipe,
                            pipe,
                        ),
                    )
                    .map(|_| fd)
            })??;

        let output_fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(
                        fd,
                        UserFileDescriptor::new(
                            Mode::WriteOnly,
                            KernelFileDescriptorType::Pipe,
                            cloned_pipe,
                        ),
                    )
                    .map(|_| fd)
            })
            .map_err(|e| {
                let _r = self.user_fd_list.remove(&input_fd);
                e
            })??;

        Ok((input_fd, output_fd))
    }

    /// Open a Fifo
    #[allow(dead_code)]
    pub fn open_fifo(&mut self, access_mode: Mode) -> SysResult<Fd> {
        if access_mode == Mode::ReadWrite {
            return Err(Errno::Eacces);
        }

        let fifo = Arc::try_new(DeadMutex::new(Fifo::new()))?;

        let fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(
                        fd,
                        UserFileDescriptor::new(access_mode, KernelFileDescriptorType::Fifo, fifo),
                    )
                    .map(|_| fd)
            })??;
        Ok(fd)
    }

    /// Open a Socket
    /// The socket type must be pass as parameter
    #[allow(dead_code)]
    pub fn open_socket(&mut self, access_mode: Mode) -> SysResult<Fd> {
        let socket = Arc::try_new(DeadMutex::new(Socket::new()))?;

        let fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(
                        fd,
                        UserFileDescriptor::new(
                            access_mode,
                            KernelFileDescriptorType::Socket,
                            socket,
                        ),
                    )
                    .map(|_| fd)
            })??;
        Ok(fd)
    }

    /// Read something from the File Descriptor:
    /// Important ! When in blocked syscall, the slice must be verified before read op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    #[allow(dead_code)]
    pub fn read(&mut self, fd: Fd, buf: &mut [u8]) -> SysResult<i32> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::Ebadf)?;

        elem.kernel.lock().read(buf)
    }

    /// Write something into the File Descriptor:
    /// Important ! When in blocked syscall, the slice must be verified before write op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    #[allow(dead_code)]
    pub fn write(&mut self, fd: Fd, buf: &[u8]) -> SysResult<i32> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::Ebadf)?;

        elem.kernel.lock().write(buf)
    }

    /// Duplicate one File Descriptor
    pub fn dup(&mut self, oldfd: Fd) -> SysResult<Fd> {
        for (key, elem) in &self.user_fd_list {
            if *key == oldfd {
                let new_elem = elem.try_clone()?;
                let newfd = self.get_lower_fd_value().ok_or::<Errno>(Errno::Emfile)?;

                self.user_fd_list.try_insert(newfd, new_elem)?;
                return Ok(newfd);
            }
        }
        Err(Errno::Ebadf)
    }

    /// Duplicate one file descriptor with possible override
    pub fn dup2(&mut self, oldfd: Fd, newfd: Fd) -> SysResult<Fd> {
        if newfd > Self::MAX_FD {
            return Err(Errno::Ebadf);
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
        Err(Errno::Ebadf)
    }

    /// Clone one file descriptor
    pub fn close_fd(&mut self, fd: Fd) -> SysResult<()> {
        self.user_fd_list.remove(&fd).ok_or::<Errno>(Errno::Ebadf)?;
        Ok(())
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

/// When fork() is invoqued, the entire FileDescriptor interface of a process is cloned
impl TryClone for FileDescriptorInterface {
    fn try_clone(&self) -> Result<Self, alloc::collections::CollectionAllocErr> {
        Ok(Self {
            user_fd_list: self.user_fd_list.try_clone()?,
        })
    }
}

/// Some boilerplate to check if all is okay
impl Drop for FileDescriptorInterface {
    fn drop(&mut self) {
        println!("FD interface droped");
    }
}
