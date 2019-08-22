//! This file contains all the stuff about Stdout special file

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

use libc_binding::Errno;

/// This structure represents a FileOperation of type Stdout
#[derive(Debug, Default)]
pub struct Stdout {
    controlling_terminal: usize,
}

/// Main implementation for Stdout
impl Stdout {
    pub fn new(controlling_terminal: usize) -> Self {
        Self {
            controlling_terminal,
        }
    }
}

/// Main Trait implementation
impl FileOperation for Stdout {
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
        print_tty!(self.controlling_terminal, "{}", unsafe {
            core::str::from_utf8_unchecked(buf)
        });
        Ok(IpcResult::Done(buf.len() as _))
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stdout {
    fn drop(&mut self) {
        //        println!("Stdout droped !");
    }
}
