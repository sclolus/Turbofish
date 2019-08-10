//! This file contains all the stuff about File Descriptors

use super::SysResult;

use errno::Errno;

use alloc::boxed::Box;
use alloc::sync::Arc;

use fallible_collections::TryClone;

use fallible_collections::boxed::FallibleBox;
use fallible_collections::btree::BTreeMap;
use fallible_collections::FallibleArc;

use sync::DeadMutex;

/// This structure represent a File Descriptor in Kernel
/// It cas be shared between process (cf Fork()) and for two user fd (cf Pipe())
// TODO: Replace bullshit inside this structure
#[derive(Debug)]
struct KernelFileDescriptor {
    nature: KernelFileDescriptorType,
    input_ref: u32,
    output_ref: u32,
    data: Box<u32>,
}

/// Here the type of the Kernel File Descriptor
#[derive(Debug)]
enum KernelFileDescriptorType {
    #[allow(dead_code)]
    Device,
    Pipe,
}

/// Some boilerplate to check if all is okay
impl Drop for KernelFileDescriptor {
    fn drop(&mut self) {
        println!("kernel File descriptor droped !");
    }
}

/// Announce the direction of an User File Descriptor
#[derive(Clone, Copy, Debug)]
enum Direction {
    Input,
    Output,
    #[allow(dead_code)]
    Bidirectionnal,
}

/// This structure design a User File Descriptor
/// We can normally clone the Arc
#[derive(Debug)]
struct UserFileDescriptor {
    direction: Direction,
    kernel: Arc<DeadMutex<KernelFileDescriptor>>,
}

/// TryClone boilerplate for UserFileDescriptor: Contains exception for Arc
impl TryClone for UserFileDescriptor {
    fn try_clone(&self) -> Result<Self, alloc::collections::CollectionAllocErr> {
        Ok(Self {
            direction: self.direction,
            // Cloning a Arc does not allocate memory. Just increments the ref count
            kernel: self.kernel.clone(),
        })
    }
}

/// Standard implementation of an user File Descriptor
impl UserFileDescriptor {
    /// When a new UserFileDescriptor is invoqued, Increment reference
    fn new(direction: Direction, kernel: Arc<DeadMutex<KernelFileDescriptor>>) -> Self {
        match direction {
            Direction::Input => kernel.lock().input_ref += 1,
            Direction::Output => kernel.lock().output_ref += 1,
            _ => unimplemented!(),
        };
        Self { direction, kernel }
    }
}

/// Drop boilerplate for an UserFileDescriptor structure. Decremente reference
impl Drop for UserFileDescriptor {
    fn drop(&mut self) {
        match self.direction {
            Direction::Input => self.kernel.lock().input_ref -= 1,
            Direction::Output => self.kernel.lock().output_ref -= 1,
            _ => unimplemented!(),
        };
    }
}

/// The User File Descriptor are sorted into a Binary Tree
/// Key is the user number and value the structure UserFileDescriptor
#[derive(Debug)]
pub struct FileDescriptorInterface {
    user_fd_list: BTreeMap<u32, UserFileDescriptor>,
}

/// Main implementation
impl FileDescriptorInterface {
    const MAX_FD: u32 = 128;

    /// Global constructor
    pub fn new() -> Self {
        Self {
            // New BTreeMap does not allocate memory
            user_fd_list: BTreeMap::new(),
        }
    }

    /// Made two File Descriptors connected with a Pipe
    pub fn new_pipe(&mut self) -> SysResult<(u32, u32)> {
        let kernel_fd = Arc::try_new(DeadMutex::new(KernelFileDescriptor {
            nature: KernelFileDescriptorType::Pipe,
            input_ref: 0,
            output_ref: 0,
            data: Box::try_new(0)?,
        }))?;
        let clone_kernel_fd = kernel_fd.clone();

        let input_fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(fd, UserFileDescriptor::new(Direction::Input, kernel_fd))
                    .map(|_| fd)
            })??;

        let output_fd = self
            .get_lower_fd_value()
            .ok_or::<Errno>(Errno::Emfile)
            .map(|fd| {
                self.user_fd_list
                    .try_insert(
                        fd,
                        UserFileDescriptor::new(Direction::Output, clone_kernel_fd),
                    )
                    .map(|_| fd)
            })
            .map_err(|e| {
                let _r = self.user_fd_list.remove(&input_fd);
                e
            })??;

        Ok((input_fd, output_fd))
    }

    /// Duplicate one File Descriptor
    pub fn dup(&mut self, oldfd: u32) -> SysResult<u32> {
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
    pub fn dup2(&mut self, oldfd: u32, newfd: u32) -> SysResult<u32> {
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
    pub fn close_fd(&mut self, fd: u32) -> SysResult<()> {
        self.user_fd_list.remove(&fd).ok_or::<Errno>(Errno::Ebadf)?;
        Ok(())
    }

    /// Get the first available File Descriptor number
    fn get_lower_fd_value(&self) -> Option<u32> {
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
