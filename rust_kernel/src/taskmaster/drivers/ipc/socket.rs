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
use core::cmp;
use fallible_collections::FallibleVec;
use libc_binding::{Errno, FileType, OpenFlags};
use sync::dead_mutex::DeadMutex;

#[derive(Debug)]
pub struct DgramMessage {
    buf: Vec<u8>,
    sender: Option<Path>,
}

impl DgramMessage {
    fn try_new(slice: &[u8], sender: Option<Path>) -> SysResult<Self> {
        let mut buf = Vec::new();
        buf.try_extend_from_slice(slice)?;
        Ok(Self { buf, sender })
    }
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

    fn send_from(&mut self, buf: &[u8], _flags: u32, sender: Option<Path>) -> SysResult<u32> {
        self.messages.try_reserve(1)?;
        self.messages.push_back(DgramMessage::try_new(buf, sender)?);
        dbg!(&self.messages);
        dbg!("send from");
        Ok(buf.len() as u32)
    }

    fn recv_from(&mut self, buf: &mut [u8], _flags: u32) -> SysResult<IpcResult<u32>> {
        let message = self.messages.pop_front();
        Ok(match message {
            Some(message) => {
                let min = cmp::min(buf.len(), message.buf.len());
                buf[0..min].copy_from_slice(&message.buf[0..min]);
                IpcResult::Done(min as u32)
            }
            None => IpcResult::Wait(0, 0),
        })
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
        unsafe {
            VFS.lock()
                .get_inode(inode_id)
                .expect("no inode wtf")
                .incr_nbr_open_file_operation();
        }
        Ok(0)
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
        if let Some(inode_id) = self.inode_id {
            VFS.lock().close_file_operation(inode_id);
        }
    }
}
