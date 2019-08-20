//! This file contains all the stuff about Stdin special file

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

use libc_binding::Errno;

use crate::terminal::{ReadResult, TERMINAL};

/// This structure represents a FileOperation of type Stdin
#[derive(Debug, Default)]
pub struct Stdin {
    controlling_terminal: usize,
}

/// Main implementation for Stdin
impl Stdin {
    pub fn new(controlling_terminal: usize) -> Self {
        Self {
            controlling_terminal,
        }
    }
}

/// Main Trait implementation
impl FileOperation for Stdin {
    fn register(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::ReadOnly);
    }
    fn unregister(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::ReadOnly);
    }
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let read_result = unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .read(buf, self.controlling_terminal)
        };

        match read_result {
            ReadResult::NonBlocking(read_count) => Ok(IpcResult::Done(read_count as _)),
            // Apply a local terminal rule: A blocked call cannot have character
            ReadResult::Blocking => Ok(IpcResult::Wait(0)),
        }
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::EBADF)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stdin {
    fn drop(&mut self) {
        //        println!("Stdin droped !");
    }
}
