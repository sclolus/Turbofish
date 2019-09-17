//! This file contains all the stuff about Socket

use super::SysResult;
use crate::taskmaster::syscall::socket;

use super::Credentials;
use super::Driver;
use super::FileOperation;
use super::InodeId;
use super::IpcResult;
use super::Path;
use super::VFS;

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryInto;
use libc_binding::{Errno, FileType, OpenFlags};
use sync::dead_mutex::DeadMutex;

#[derive(Debug)]
pub struct DgramMessage {
    buf: Vec<u8>,
    sender: Option<Path>,
}

#[derive(Debug)]
pub struct SocketDriver {
    messages: VecDeque<DgramMessage>,
}

impl SocketDriver {
    pub fn try_new() -> SysResult<Self> {
        Ok(Self {
            messages: VecDeque::new(),
        })
    }
}

impl Driver for SocketDriver {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Err(Errno::ENOSYS)
    }

    fn send_from(&mut self, buf: &[u8], flags: u32, sender: Option<Path>) -> SysResult<u32> {
        dbg!("send from");
        Err(Errno::ENOSYS)
    }
}

/// This structure represents a FileOperation of type Socket
#[derive(Debug)]
pub struct Socket {
    domain: socket::Domain,
    socket_type: socket::SocketType,
    inode_id: Option<InodeId>,
}

/// Main implementation for Socket
impl Socket {
    pub fn new(domain: socket::Domain, socket_type: socket::SocketType) -> SysResult<Self> {
        Ok(Self {
            domain,
            socket_type,
            inode_id: None,
        })
    }
}

/// Main Trait implementation
impl FileOperation for Socket {
    fn register(&mut self, _flags: OpenFlags) {}
    fn unregister(&mut self, _flags: OpenFlags) {}
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
    fn bind(&mut self, cwd: &Path, creds: &Credentials, sockaddr: Path) -> SysResult<u32> {
        let inode_id = VFS
            .lock()
            .mknod(cwd, creds, sockaddr, FileType::UNIX_SOCKET)?;
        self.inode_id = Some(inode_id);
        Ok(0)
        //     VFS.lock().get_driver(dest_addr).send();
    }

    fn send_to(&mut self, buf: &[u8], flags: u32, sockaddr_opt: Option<Path>) -> SysResult<u32> {
        match sockaddr_opt {
            Some(sockaddr) => {
                let mut vfs = VFS.lock();
                let inode_id = vfs.inode_id_from_absolute_path(sockaddr)?;
                let driver = vfs.get_driver(inode_id)?;
                driver.send_from(buf, flags, None)
            }
            None => unimplemented!(),
        }
    }

    fn recv_from(&mut self, buf: &mut [u8], flags: u32) -> SysResult<IpcResult<u32>> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id.ok_or(Errno::EINVAL)?)?;
        driver.recv_from(buf, flags)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Socket {
    fn drop(&mut self) {
        println!("Socket droped !");
    }
}
