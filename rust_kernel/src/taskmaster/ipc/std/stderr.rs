//! This file contains all the stuff about Stderr special file

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

use libc_binding::Errno;

use crate::terminal::ansi_escape_code::Colored;

/// This structure represents a FileOperation of type Stderr
#[derive(Debug, Default)]
pub struct Stderr {
    controlling_terminal: usize,
}

/// Main implementation for Stderr
impl Stderr {
    pub fn new(controlling_terminal: usize) -> Self {
        Self {
            controlling_terminal,
        }
    }
}

/// Main Trait implementation
impl FileOperation for Stderr {
    fn register(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::WriteOnly);
    }
    fn unregister(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::WriteOnly);
    }
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::EBADF)
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        print_tty!(
            self.controlling_terminal,
            "{}",
            unsafe { core::str::from_utf8_unchecked(buf) }.yellow()
        );
        Ok(IpcResult::Done(buf.len() as _))
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stderr {
    fn drop(&mut self) {
        //        println!("Stderr droped !");
    }
}
