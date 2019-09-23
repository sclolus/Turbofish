use super::SysResult;
use crate::taskmaster::syscall::socket;

use super::get_file_op_uid;
use super::Credentials;
use super::FileOperation;
use super::InodeId;
use super::IpcResult;
use super::Path;
use super::SocketDriver;
use super::Whom;
use super::VFS;

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cmp;
use fallible_collections::{FallibleBox, FallibleVec, TryClone};
use libc_binding::{Errno, FileType, OpenFlags};
use messaging::MessageTo;

#[derive(Debug)]
pub struct SocketDgramDriver {
    /// the messaging for the dgram message
    messages: VecDeque<DgramMessage>,
    file_op_uid: usize,
}

#[derive(Debug)]
pub struct DgramMessage {
    /// the data of the message
    buf: Vec<u8>,
    /// the sender
    sender: Option<Path>,
}

impl DgramMessage {
    fn try_new(slice: &[u8], sender: Option<Path>) -> SysResult<Self> {
        let mut buf = Vec::new();
        buf.try_extend_from_slice(slice)?;
        Ok(Self { buf, sender })
    }
}

impl SocketDgramDriver {
    pub fn try_new() -> SysResult<Self> {
        let file_op_uid = get_file_op_uid();
        Ok(Self {
            messages: VecDeque::new(),
            file_op_uid,
        })
    }

    /// send a message on the socket from the sender `sender`
    pub(super) fn send_from(
        &mut self,
        buf: &[u8],
        _flags: u32,
        sender: Option<Path>,
    ) -> SysResult<IpcResult<u32>> {
        self.messages.try_reserve(1)?;
        self.messages.push_back(DgramMessage::try_new(buf, sender)?);
        unsafe {
            messaging::send_message(MessageTo::Reader {
                uid_file_op: self.file_op_uid,
            });
        }
        Ok(IpcResult::Done(buf.len() as u32))
    }

    pub(super) fn recv_from(
        &mut self,
        buf: &mut [u8],
        _flags: u32,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        let message = self.messages.pop_front();
        Ok(match message {
            Some(message) => {
                let min = cmp::min(buf.len(), message.buf.len());
                buf[0..min].copy_from_slice(&message.buf[0..min]);
                IpcResult::Done((min as u32, message.sender))
            }
            // fill the file_op_uid field of IpcResult::Wait
            None => IpcResult::Wait((0, None), self.file_op_uid),
        })
    }
}

/// This structure represents a FileOperation of type Socket
#[derive(Debug)]
pub struct SocketDgram {
    /// we only handle AF_UNIX domain
    domain: socket::Domain,
    /// the type of the socket(Dgram, Stream, SeqPacket)
    socket_type: socket::SocketType,
    /// the inode id of the socket if it is binded
    inode_id: InodeId,
    /// the address of the socket file if the socket is binded
    path: Option<Path>,
    /// the peer address if the socket is connected
    peer_address: Option<Path>,
    /// the peer inode id if the socket is connected
    peer_inode_id: Option<InodeId>,
}

/// Main implementation for Socket
impl SocketDgram {
    pub fn new(domain: socket::Domain, socket_type: socket::SocketType) -> SysResult<Self> {
        let mut vfs = VFS.lock();
        let inode_id = vfs.add_orphan_driver(Box::try_new(SocketDriver::try_new(socket_type)?)?)?;
        unsafe {
            vfs.get_inode(inode_id)
                .expect("no inode wtf")
                .incr_nbr_open_file_operation();
        }
        Ok(Self {
            domain,
            socket_type,
            inode_id: inode_id,
            path: None,
            peer_address: None,
            peer_inode_id: None,
        })
    }
}

/// Main Trait implementation
impl FileOperation for SocketDgram {
    fn register(&mut self, _flags: OpenFlags) {}
    fn unregister(&mut self, _flags: OpenFlags) {}
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }

    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }

    fn bind(&mut self, cwd: &Path, creds: &Credentials, sockaddr: Path) -> SysResult<u32> {
        let mut vfs = VFS.lock();
        let inode_id = vfs.mknod(cwd, creds, sockaddr.try_clone()?, FileType::UNIX_SOCKET)?;

        let driver = vfs.remove_orphan_driver(self.inode_id)?;
        let inode = vfs
            .get_inode(inode_id)
            .expect("inode should have been created by mknod");
        inode.set_driver(driver);
        self.inode_id = inode_id;
        self.path = Some(sockaddr);
        unsafe {
            // we increment the counter of file operation on the
            // inode manualy, because we didn't call open on the
            // inode
            vfs.get_inode(inode_id)
                .expect("no inode wtf")
                .incr_nbr_open_file_operation();
        }
        Ok(0)
    }

    fn connect(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        sockaddr: Path,
    ) -> SysResult<IpcResult<()>> {
        let mut vfs = VFS.lock();
        let absolute_path = vfs.resolve_path(cwd, &sockaddr)?;
        let inode_id = vfs.inode_id_from_absolute_path(&absolute_path)?;
        self.peer_address = Some(absolute_path);
        self.peer_inode_id = Some(inode_id);
        Ok(IpcResult::Done(()))
    }

    fn send_to(
        &mut self,
        buf: &[u8],
        flags: u32,
        sockaddr_opt: Option<Path>,
    ) -> SysResult<IpcResult<u32>> {
        let sockaddr = match sockaddr_opt {
            Some(sockaddr) => sockaddr,
            None => {
                if let Some(peer_addr) = &self.peer_address {
                    peer_addr.try_clone()?
                } else {
                    return Err(Errno::EDESTADDRREQ);
                }
            }
        };
        let mut vfs = VFS.lock();
        let inode_id = vfs.inode_id_from_absolute_path(&sockaddr)?;
        let driver = vfs.get_driver(inode_id)?;
        driver.send_from(buf, flags, self.path.try_clone()?, Whom::Client)
    }

    fn recv_from(
        &mut self,
        buf: &mut [u8],
        flags: u32,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id)?;
        driver.recv_from(buf, flags, Whom::Client)
    }

    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for SocketDgram {
    fn drop(&mut self) {
        println!("Socket droped !");
        VFS.lock().close_file_operation(self.inode_id);
    }
}
