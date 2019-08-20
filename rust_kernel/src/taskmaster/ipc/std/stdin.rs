//! This file contains all the stuff about Stdin special file

use super::SysResult;

use super::IpcResult;
use super::KernelFileDescriptor;
use super::Mode;

use errno::Errno;

use crate::terminal::{ReadResult, TERMINAL};

/// This structure represents a KernelFileDescriptor of type Stdin
#[derive(Debug, Default)]
pub struct Stdin {}

/// Main implementation for Stdin
impl Stdin {
    pub fn new() -> Self {
        Self {}
    }
}

/// Main Trait implementation
impl KernelFileDescriptor for Stdin {
    fn register(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::ReadOnly);
    }
    fn unregister(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::ReadOnly);
    }
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        // TODO: change that, read on tty 1 for the moment
        let read_result = unsafe { TERMINAL.as_mut().expect("WTF").read(buf, 1) };

        match read_result {
            ReadResult::NonBlocking(read_count) => Ok(IpcResult::Done(read_count as _)),
            // Apply a local terminal rule: A blocked call cannot have character
            ReadResult::Blocking => Ok(IpcResult::Wait(0)),
        }
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::Ebadf)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stdin {
    fn drop(&mut self) {
        println!("Stdin droped !");
    }
}
