//! This file contains all the stuff about Stderr special file

use super::SysResult;

use super::IpcResult;
use super::KernelFileDescriptor;
use super::Mode;

use errno::Errno;

use crate::terminal::ansi_escape_code::Colored;

/// This structure represents a KernelFileDescriptor of type Stderr
#[derive(Debug, Default)]
pub struct Stderr {}

/// Main implementation for Stderr
impl Stderr {
    pub fn new() -> Self {
        Self {}
    }
}

/// Main Trait implementation
impl KernelFileDescriptor for Stderr {
    fn register(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::WriteOnly);
    }
    fn unregister(&mut self, access_mode: Mode) {
        assert_eq!(access_mode, Mode::WriteOnly);
    }
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        Err(Errno::Ebadf)
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unsafe {
            print!("{}", core::str::from_utf8_unchecked(buf).yellow());
        }
        Ok(IpcResult::cont(buf.len() as _))
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stderr {
    fn drop(&mut self) {
        println!("Stderr droped !");
    }
}
