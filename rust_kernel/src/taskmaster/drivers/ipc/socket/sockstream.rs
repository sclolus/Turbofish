use super::SysResult;
use crate::taskmaster::syscall::socket;

use super::get_file_op_uid;
use super::Buf;
use super::Credentials;
use super::FileOperation;
use super::InodeId;
use super::IpcResult;
use super::Path;
use super::SocketDriver;
use super::VFS;

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::cmp;
use fallible_collections::{FallibleBox, FallibleVec, TryClone};
use libc_binding::{Errno, FileType, OpenFlags};
use messaging::MessageTo;

#[derive(Debug)]
struct Connection {
    inode_id: InodeId,
    path: Option<Path>,
}

/// this correspond to who called the fonction
#[derive(Debug, Copy, Clone)]
pub enum Whom {
    /// is this the client ?
    Client,
    /// or the server ?
    Server,
}

use Whom::*;

#[derive(Debug, Default)]
struct StreamedMessaging {
    buf: Buf,
    index: usize,
}

impl StreamedMessaging {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
pub struct Packet {
    /// the data of the message
    buf: Vec<u8>,
    // /// the sender
    // sender: Option<Path>,
}

impl Packet {
    fn try_new(slice: &[u8]) -> SysResult<Self> {
        let mut buf = Vec::new();
        buf.try_extend_from_slice(slice)?;
        Ok(Self { buf })
    }
}

#[derive(Debug)]
struct PacketedMessaging {
    messages: VecDeque<Packet>,
}

impl PacketedMessaging {
    fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
enum Messaging {
    Streamed(StreamedMessaging),
    Packeted(PacketedMessaging),
}

use Messaging::*;

impl Messaging {
    fn try_new(socket_type: socket::SocketType) -> SysResult<Self> {
        Ok(match socket_type {
            socket::SocketType::SockStream => Self::Streamed(StreamedMessaging::new()),
            socket::SocketType::SockSeqPacket => Self::Packeted(PacketedMessaging::new()),
            _ => panic!("wrong socket type"),
        })
    }
}

#[derive(Debug)]
pub struct SocketStreamDriver {
    socket_type: socket::SocketType,
    listen_queue: Option<VecDeque<Connection>>,
    file_op_uid: usize,
    messaging_to_server: Messaging,
    messaging_to_client: Messaging,
    // buf_to_server: Buf,
    // to_server_index: usize,
    // buf_to_client: Buf,
    // to_client_index: usize,
}

impl SocketStreamDriver {
    const DEFAULT_LISTEN_QUEUE_CAPACITY: usize = 10;

    pub fn try_new(socket_type: socket::SocketType) -> SysResult<Self> {
        let file_op_uid = get_file_op_uid();
        Ok(SocketStreamDriver {
            socket_type,
            listen_queue: None,
            file_op_uid,
            messaging_to_client: Messaging::try_new(socket_type)?,
            messaging_to_server: Messaging::try_new(socket_type)?,
            // buf_to_server: Default::default(),
            // to_server_index: Default::default(),
            // buf_to_client: Default::default(),
            // to_client_index: Default::default(),
        })
    }

    /// send a message on the socket from the sender `sender`
    pub(super) fn send_from(
        &mut self,
        buf: &[u8],
        _flags: u32,
        _sender: Option<Path>,
        whom: Whom,
    ) -> SysResult<IpcResult<u32>> {
        match self.socket_type {
            socket::SocketType::SockStream => {
                let (self_buf, current_index) = match whom {
                    Client => match &mut self.messaging_to_server {
                        Streamed(StreamedMessaging { buf, index }) => (buf, index),
                        _ => panic!("wrong socket type"),
                    },
                    Server => match &mut self.messaging_to_client {
                        Streamed(StreamedMessaging { buf, index }) => (buf, index),
                        _ => panic!("wrong socket type"),
                    },
                };
                let min = cmp::min(buf.len(), Buf::BUF_SIZE - *current_index);

                self_buf[*current_index..*current_index + min].copy_from_slice(&buf[..min]);
                *current_index += min;

                // If the writer has writed something into the pipe...
                if min > 0 {
                    unsafe {
                        messaging::send_message(MessageTo::Reader {
                            uid_file_op: self.file_op_uid,
                        });
                    }
                }
                if min == buf.len() {
                    Ok(IpcResult::Done(min as _))
                } else {
                    Ok(IpcResult::Wait(min as _, self.file_op_uid))
                }
            }
            socket::SocketType::SockSeqPacket => {
                let messages = match whom {
                    Client => match &mut self.messaging_to_server {
                        Packeted(PacketedMessaging { messages }) => messages,
                        _ => panic!("wrong socket type"),
                    },
                    Server => match &mut self.messaging_to_client {
                        Packeted(PacketedMessaging { messages }) => messages,
                        _ => panic!("wrong socket type"),
                    },
                };
                messages.try_reserve(1)?;
                messages.push_back(Packet::try_new(buf)?);
                unsafe {
                    messaging::send_message(MessageTo::Reader {
                        uid_file_op: self.file_op_uid,
                    });
                }
                Ok(IpcResult::Done(buf.len() as u32))
            }
            _ => panic!("wrong socket type"),
        }
    }

    pub(super) fn recv_from(
        &mut self,
        buf: &mut [u8],
        _flags: u32,
        whom: Whom,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        match self.socket_type {
            socket::SocketType::SockStream => {
                let (self_buf, current_index) = match whom {
                    Client => match &mut self.messaging_to_client {
                        Streamed(StreamedMessaging { buf, index }) => (buf, index),
                        _ => panic!("wrong socket type"),
                    },
                    Server => match &mut self.messaging_to_server {
                        Streamed(StreamedMessaging { buf, index }) => (buf, index),
                        _ => panic!("wrong socket type"),
                    },
                };

                if *current_index == 0 {
                    // if self.output_ref == 0 {
                    //     // Writers are gone, returns immediatly
                    //     return Ok(IpcResult::Done(0));
                    // } else {
                    // Waiting for a writer
                    return Ok(IpcResult::Wait((0, None), self.file_op_uid));
                    // }
                }

                let min = cmp::min(buf.len(), *current_index);

                // memcpy(buf, self_buf, MIN(buf.len(), *current_index)
                buf[..min].copy_from_slice(&self_buf[..min]);

                // memcpy(self_buf, self_buf + MIN(buf.len(), *current_index), *current_index - MIN(buf.len(), *current_index))
                self_buf.copy_within(min..*current_index, 0);
                *current_index -= min;

                unsafe {
                    messaging::send_message(MessageTo::Writer {
                        uid_file_op: self.file_op_uid,
                    });
                }
                Ok(IpcResult::Done((min as _, None)))
            }
            socket::SocketType::SockSeqPacket => {
                let messages = match whom {
                    Client => match &mut self.messaging_to_client {
                        Packeted(PacketedMessaging { messages }) => messages,
                        _ => panic!("wrong socket type"),
                    },
                    Server => match &mut self.messaging_to_server {
                        Packeted(PacketedMessaging { messages }) => messages,
                        _ => panic!("wrong socket type"),
                    },
                };
                let message = messages.pop_front();
                Ok(match message {
                    Some(message) => {
                        let min = cmp::min(buf.len(), message.buf.len());
                        buf[0..min].copy_from_slice(&message.buf[0..min]);
                        IpcResult::Done((min as u32, None))
                    }
                    // fill the file_op_uid field of IpcResult::Wait
                    None => IpcResult::Wait((0, None), self.file_op_uid),
                })
            }
            _ => panic!("wrong socket type"),
        }
    }

    pub(super) fn connect(
        &mut self,
        addr: Option<Path>,
        inode_id: InodeId,
    ) -> SysResult<IpcResult<()>> {
        // If the connection cannot be established immediately and
        // O_NONBLOCK is not set for the file descriptor for the
        // socket, connect() shall block for up to an unspecified
        // timeout interval until the connection is established
        let listen_queue = self.listen_queue.as_mut().ok_or(Errno::ECONNREFUSED)?;
        if listen_queue.len() == listen_queue.capacity() {
            return Err(Errno::ECONNREFUSED);
        }
        listen_queue.push_back(Connection {
            path: addr,
            inode_id,
        });

        unsafe {
            messaging::send_message(MessageTo::Accepter {
                uid_file_op: self.file_op_uid,
            });
        }
        Ok(IpcResult::Done(()))
        // IpcResult::Wait((), unimplemented!())
    }

    pub(super) fn listen(&mut self, mut backlog: i32) -> SysResult<()> {
        // A backlog argument of 0 may allow the socket to accept connections,
        // in which case the length of the listen queue may be set to an
        // implementation-defined minimum value.
        if backlog <= 0 {
            backlog = Self::DEFAULT_LISTEN_QUEUE_CAPACITY as i32;
        }
        let mut listen_queue = VecDeque::new();
        listen_queue.try_reserve(backlog as usize)?;
        self.listen_queue = Some(listen_queue);
        Ok(())
    }

    pub(super) fn accept(&mut self) -> SysResult<IpcResult<(Option<SocketStream>)>> {
        // If the listen queue is empty of connection requests and
        // O_NONBLOCK is not set on the file descriptor for the socket,
        // accept() shall block until a connection is present.
        Ok(
            if let Some(conn) = self.listen_queue.as_mut().ok_or(Errno::EINVAL)?.pop_front() {
                let socket_stream = SocketStream::from_driver(conn.inode_id, conn.path);
                IpcResult::Done(Some(socket_stream))
            } else {
                IpcResult::Wait(None, self.file_op_uid)
            },
        )
    }
}

/// This structure represents a FileOperation of type Socket
#[derive(Debug)]
pub struct SocketStream {
    /// we only handle AF_UNIX domain
    domain: socket::Domain,
    /// the type of the socket(Dgram, Stream, SeqPacket)
    socket_type: socket::SocketType,
    /// the inode id of the socket if it is binded
    inode_id: InodeId,
    /// the address of the socket file if the socket is binded
    pub path: Option<Path>,
    /// the peer address if the socket is connected
    peer_address: Option<Path>,
    /// the peer inode id if the socket is connected
    peer_inode_id: Option<InodeId>,
    whom: Whom,
}

/// Main implementation for Socket
impl SocketStream {
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
            inode_id,
            path: None,
            peer_address: None,
            peer_inode_id: None,
            whom: Whom::Client,
        })
    }

    fn from_driver(inode_id: InodeId, path: Option<Path>) -> Self {
        unsafe {
            VFS.force_unlock();
            VFS.lock()
                .get_inode(inode_id)
                .expect("no inode wtf")
                .incr_nbr_open_file_operation();
        }
        Self {
            domain: socket::Domain::AfUnix,
            socket_type: socket::SocketType::SockStream,
            inode_id,
            path,
            peer_address: None,
            peer_inode_id: None,
            whom: Whom::Server,
        }
    }

    fn send_to_without_creds(&mut self, buf: &[u8], flags: u32) -> SysResult<IpcResult<u32>> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id)?;
        driver.send_from(buf, flags, self.path.try_clone()?, self.whom)
    }
}

/// Main Trait implementation
impl FileOperation for SocketStream {
    fn register(&mut self, _flags: OpenFlags) {}
    fn unregister(&mut self, _flags: OpenFlags) {}

    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        Ok(match self.recv_from(buf, 0)? {
            IpcResult::Done((readen_bytes, _path)) => IpcResult::Done(readen_bytes),
            IpcResult::Wait((readen_bytes, _path), file_op_uid) => {
                IpcResult::Wait(readen_bytes, file_op_uid)
            }
        })
    }

    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        self.send_to_without_creds(buf, 0)
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
        creds: &Credentials,
        sockaddr: Path,
    ) -> SysResult<IpcResult<()>> {
        let mut vfs = VFS.lock();
        let absolute_path = vfs.resolve_path(cwd, creds, &sockaddr)?;
        let inode_id = vfs.inode_id_from_absolute_path(&absolute_path, creds)?;
        // self.peer_address = Some(absolute_path);
        // self.peer_inode_id = Some(inode_id);
        let driver = vfs.get_driver(inode_id)?;
        // unimplemented!();
        driver.connect(self.path.try_clone()?, self.inode_id)
    }

    fn send_to(
        &mut self,
        _creds: &Credentials,
        buf: &[u8],
        flags: u32,
        _sockaddr_opt: Option<Path>,
    ) -> SysResult<IpcResult<u32>> {
        self.send_to_without_creds(buf, flags)
    }

    fn recv_from(
        &mut self,
        buf: &mut [u8],
        flags: u32,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id)?;
        driver.recv_from(buf, flags, self.whom)
    }

    fn listen(&mut self, backlog: i32) -> SysResult<()> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id)?;
        driver.listen(backlog)
    }

    fn accept(&mut self) -> SysResult<IpcResult<Option<SocketStream>>> {
        let mut vfs = VFS.lock();
        let driver = vfs.get_driver(self.inode_id)?;
        driver.accept()
    }

    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for SocketStream {
    fn drop(&mut self) {
        println!("Socket droped !");
        VFS.lock().close_file_operation(self.inode_id);
    }
}
