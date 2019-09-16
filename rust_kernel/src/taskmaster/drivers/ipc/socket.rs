//! This file contains all the stuff about Socket

use super::SysResult;
use crate::taskmaster::syscall::socket;

use super::FileOperation;
use super::IpcResult;

use libc_binding::OpenFlags;

/// This structure represents a FileOperation of type Socket
#[derive(Debug)]
pub struct Socket {
    domain: socket::Domain,
    socket_type: socket::SocketType,
}

/// Main implementation for Socket
impl Socket {
    pub fn new(domain: socket::Domain, socket_type: socket::SocketType) -> SysResult<Self> {
        Ok(Self {
            domain,
            socket_type,
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
}

/// Some boilerplate to check if all is okay
impl Drop for Socket {
    fn drop(&mut self) {
        println!("Socket droped !");
    }
}
