//! This file contains all the stuff about Fifo

use super::SysResult;

use super::KernelFileDescriptor;
use super::Mode;

/// This structure represents a KernelFileDescriptor of type Fifo
#[derive(Debug, Default)]
pub struct Fifo {}

/// Main implementation for Fifo
impl Fifo {
    pub fn new() -> Self {
        Self {}
    }
}

/// Main Trait implementation
impl KernelFileDescriptor for Fifo {
    fn register(&mut self, _access_mode: Mode) {}
    fn unregister(&mut self, _access_mode: Mode) {}
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<i32> {
        Ok(0)
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<i32> {
        Ok(0)
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Fifo {
    fn drop(&mut self) {
        println!("Fifo droped !");
    }
}
