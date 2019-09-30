//! This file contains all the stuff about Socket

use super::SysResult;
use crate::taskmaster::syscall::socket;

use super::get_file_op_uid;
use super::Buf;
use super::Credentials;
use super::Driver;
use super::FileOperation;
use super::InodeId;
use super::IpcResult;
use super::Path;
use super::VFS;

use alloc::sync::Arc;
use libc_binding::{Errno, OpenFlags};
use sync::dead_mutex::DeadMutex;

mod sockdgram;
pub use sockdgram::SocketDgram;
use sockdgram::SocketDgramDriver;

mod sockstream;
use sockstream::ConnectedSocketDriver;
pub use sockstream::{ConnectedSocket, Whom};

#[derive(Debug)]
pub enum SocketDriver {
    Stream(ConnectedSocketDriver),
    Dgram(SocketDgramDriver),
}

impl SocketDriver {
    pub fn try_new(socket_type: socket::SocketType) -> SysResult<Self> {
        Ok(match socket_type {
            socket::SocketType::SockStream | socket::SocketType::SockSeqPacket => {
                Self::Stream(ConnectedSocketDriver::try_new(socket_type)?)
            }
            socket::SocketType::SockDgram => Self::Dgram(SocketDgramDriver::try_new()?),
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

    fn send_from(
        &mut self,
        buf: &[u8],
        flags: u32,
        sender: Option<Path>,
        whom: Whom,
    ) -> SysResult<IpcResult<u32>> {
        use SocketDriver::*;
        match self {
            Stream(driver) => driver.send_from(buf, flags, sender, whom),
            Dgram(driver) => driver.send_from(buf, flags, sender),
        }
    }

    fn recv_from(
        &mut self,
        buf: &mut [u8],
        flags: u32,
        whom: Whom,
    ) -> SysResult<IpcResult<(u32, Option<Path>)>> {
        use SocketDriver::*;
        match self {
            Stream(driver) => driver.recv_from(buf, flags, whom),
            Dgram(driver) => driver.recv_from(buf, flags),
        }
    }

    fn connect(&mut self, addr: Option<Path>, inode_id: InodeId) -> SysResult<IpcResult<()>> {
        use SocketDriver::*;
        match self {
            Stream(driver) => driver.connect(addr, inode_id),
            Dgram(_driver) => {
                // A Dgram Driver does not support connection
                return Err(Errno::EINVAL);
            }
        }
    }

    fn listen(&mut self, backlog: i32) -> SysResult<()> {
        use SocketDriver::*;
        match self {
            Stream(driver) => driver.listen(backlog),
            Dgram(_driver) => {
                // A Dgram Driver does not support connection
                return Err(Errno::EINVAL);
            }
        }
    }

    fn accept(&mut self) -> SysResult<IpcResult<Option<ConnectedSocket>>> {
        use SocketDriver::*;
        match self {
            Stream(driver) => driver.accept(),
            Dgram(_driver) => {
                // A Dgram Driver does not support connection
                return Err(Errno::EINVAL);
            }
        }
    }
}
