use super::drivers::FileOperation;
use super::syscall::socket;
use super::thread_group::Credentials;
use super::vfs::Path;
use super::IpcResult;
/// The User File Descriptor are sorted into a Binary Tree
/// Key is the user number and value the structure FileDescriptor
use super::SysResult;
use super::VFS;

use core::convert::TryFrom;

use libc_binding::{Errno, FileType, OpenFlags};

use super::drivers::ipc::{ConnectedSocket, Pipe, SocketDgram};
use alloc::sync::Arc;

use fallible_collections::btree::BTreeMap;
use fallible_collections::FallibleArc;
use fallible_collections::TryClone;

use try_clone_derive::TryClone;

use sync::{DeadMutex, DeadMutexGuard};

pub type Fd = u32;

#[derive(Debug, TryClone)]
pub struct FileDescriptorInterface {
    user_fd_list: BTreeMap<Fd, FileDescriptor>,
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

    /// Clear all the owned content into the File Descriptor Interface
    pub fn delete(&mut self) {
        self.user_fd_list.clear();
    }

    pub fn get_file_operation(&self, fd: Fd) -> SysResult<DeadMutexGuard<dyn FileOperation>> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::EBADF)?;
        Ok(elem.file_operation.lock())
    }

    /// Open a file and give a file descriptor
    pub fn open(
        &mut self,
        cwd: &Path,
        creds: &Credentials,
        filename: &str,
        flags: OpenFlags,
        mode: FileType,
    ) -> SysResult<IpcResult<Fd>> {
        let path = super::vfs::Path::try_from(filename)?;

        let file_operator = VFS.lock().open(cwd, creds, path, flags, mode)?;
        match file_operator {
            IpcResult::Done(file_operator) => {
                let fd = self.insert_user_fd(flags, file_operator)?;
                Ok(IpcResult::Done(fd))
            }
            IpcResult::Wait(file_operator, file_op_uid) => {
                let fd = self.insert_user_fd(flags, file_operator)?;
                Ok(IpcResult::Wait(fd, file_op_uid))
            }
        }
    }

    /// Clone one file descriptor
    pub fn close_fd(&mut self, fd: Fd) -> SysResult<()> {
        self.user_fd_list.remove(&fd).ok_or::<Errno>(Errno::EBADF)?;
        Ok(())
    }

    /// Read something from the File Descriptor: Can block
    /// Important ! When in blocked syscall, the slice must be verified before read op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    pub fn read(&mut self, fd: Fd, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::EBADF)?;

        if !elem.flags.is_open_for_read() {
            return Err(Errno::EBADF);
        }
        elem.file_operation.lock().read(buf)
    }

    /// Write something into the File Descriptor: Can block
    /// Important ! When in blocked syscall, the slice must be verified before write op and
    /// we have fo find a solution to avoid the DeadLock when multiple access to fd occured
    pub fn write(&mut self, fd: Fd, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        let elem = self.user_fd_list.get(&fd).ok_or::<Errno>(Errno::EBADF)?;

        if !elem.flags.is_open_for_write() {
            return Err(Errno::EBADF);
        }
        elem.file_operation.lock().write(buf)
    }

    /// Made two File Descriptors connected with a Pipe
    pub fn new_pipe(&mut self) -> SysResult<(Fd, Fd)> {
        let pipe = Arc::try_new(DeadMutex::new(Pipe::new()))?;
        let cloned_pipe = pipe.clone();

        let input_fd = self.insert_user_fd(OpenFlags::O_RDONLY, pipe)?;
        let output_fd = self
            .insert_user_fd(OpenFlags::O_WRONLY, cloned_pipe)
            .map_err(|e| {
                let _r = self.user_fd_list.remove(&input_fd);
                e
            })?;

        Ok((input_fd, output_fd))
    }

    /// Duplicate one File Descriptor
    pub fn dup(&mut self, oldfd: Fd, minimum: Option<Fd>) -> SysResult<Fd> {
        if let Some(elem) = self.user_fd_list.get(&oldfd) {
            let new_elem = elem.try_clone()?;
            let newfd = self
                .get_lower_fd_value(minimum.unwrap_or(0))
                .ok_or::<Errno>(Errno::EMFILE)?;

            self.user_fd_list.try_insert(newfd, new_elem)?;
            return Ok(newfd);
        }
        Err(Errno::EBADF)
    }

    /// Duplicate one file descriptor with possible override
    pub fn dup2(&mut self, oldfd: Fd, newfd: Fd) -> SysResult<Fd> {
        if newfd > Self::MAX_FD {
            return Err(Errno::EBADF);
        }

        // If oldfd is not a valid file descriptor, then the call fails, and newfd is not closed.
        if let Some(elem) = self.user_fd_list.get(&oldfd) {
            let new_elem = elem.try_clone()?;
            let _r = self.close_fd(newfd);

            self.user_fd_list.try_insert(newfd, new_elem)?;
            return Ok(newfd);
        }
        Err(Errno::EBADF)
    }

    /// Insert a new User File Descriptor atached to a Kernel File Descriptor:
    /// return value: User File Descriptor index
    fn insert_user_fd(
        &mut self,
        flags: OpenFlags,
        file_operation: Arc<DeadMutex<dyn FileOperation>>,
    ) -> SysResult<Fd> {
        let user_fd = self.get_lower_fd_value(0).ok_or::<Errno>(Errno::EMFILE)?;
        self.user_fd_list
            .try_insert(user_fd, FileDescriptor::new(flags, file_operation))?;
        Ok(user_fd)
    }

    /// Get the first available File Descriptor number that is superior to `minimum`
    fn get_lower_fd_value(&self, minimum: Fd) -> Option<Fd> {
        let mut lower_fd = minimum;

        for &key in self.user_fd_list.keys().skip_while(|&key| *key < minimum) {
            if lower_fd < key {
                break;
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

    /// Open a Socket
    pub fn open_socket(
        &mut self,
        domain: socket::Domain,
        socket_type: socket::SocketType,
    ) -> SysResult<Fd> {
        let file_operator: Arc<DeadMutex<dyn FileOperation>> = match socket_type {
            socket::SocketType::SockDgram => {
                Arc::try_new(DeadMutex::new(SocketDgram::new(domain, socket_type)?))?
            }
            socket::SocketType::SockStream | socket::SocketType::SockSeqPacket => {
                Arc::try_new(DeadMutex::new(ConnectedSocket::new(domain, socket_type)?))?
            }
        };
        self.insert_user_fd(OpenFlags::O_RDWR, file_operator)
    }

    pub fn accept_socket(&mut self, socket_fd: u32) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        let mut file_operation = self.get_file_operation(socket_fd)?;
        let res = file_operation.accept()?;
        drop(file_operation);

        Ok(match res {
            IpcResult::Wait(_res, file_op_uid) => IpcResult::Wait((0, None), file_op_uid),
            IpcResult::Done(socket_stream) => {
                let socket_stream = socket_stream.expect("socket stream should be there");
                let sender_path = socket_stream.path.try_clone()?;
                let new_fd = self.insert_user_fd(
                    OpenFlags::O_RDWR,
                    Arc::new(DeadMutex::new(socket_stream)) as Arc<DeadMutex<dyn FileOperation>>,
                )?;
                IpcResult::Done((new_fd, sender_path))
            }
        })
    }
}

/// Some boilerplate to check if all is okay
impl Drop for FileDescriptorInterface {
    fn drop(&mut self) {
        //         println!("FD interface droped");
    }
}

/// This structure design a User File Descriptor
/// We can normally clone the Arc
#[derive(Debug)]
struct FileDescriptor {
    flags: OpenFlags,
    file_operation: Arc<DeadMutex<dyn FileOperation>>,
}

use alloc::collections::CollectionAllocErr;

/// TryClone Boilerplate. The ref counter of the FileOperation must be incremented when Cloning
impl TryClone for FileDescriptor {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        self.file_operation.lock().register(self.flags);
        Ok(Self {
            flags: self.flags.clone(),
            file_operation: self.file_operation.clone(),
        })
    }
}

/// Standard implementation of an user File Descriptor
impl FileDescriptor {
    /// When a new FileDescriptor is invoqued, Increment reference
    fn new(flags: OpenFlags, file_operation: Arc<DeadMutex<dyn FileOperation>>) -> Self {
        file_operation.lock().register(flags);
        Self {
            flags,
            file_operation,
        }
    }
}

/// Drop boilerplate for an FileDescriptor structure. Decremente reference
impl Drop for FileDescriptor {
    fn drop(&mut self) {
        self.file_operation.lock().unregister(self.flags);
    }
}
