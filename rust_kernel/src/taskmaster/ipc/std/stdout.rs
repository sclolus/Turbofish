//! This file contains all the stuff about Stdout special file

use super::SysResult;

use super::IpcResult;
use super::KernelFileDescriptor;
use super::Mode;

use errno::Errno;

/// This structure represents a KernelFileDescriptor of type Stdout
#[derive(Debug, Default)]
pub struct Stdout {}

/// Main implementation for Stdout
impl Stdout {
    pub fn new() -> Self {
        Self {}
    }
}

/// Main Trait implementation
impl KernelFileDescriptor for Stdout {
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
            print!("{}", core::str::from_utf8_unchecked(buf));
        }
        Ok(IpcResult::Done(buf.len() as _))
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Stdout {
    fn drop(&mut self) {
        println!("Stdout droped !");
    }
}
