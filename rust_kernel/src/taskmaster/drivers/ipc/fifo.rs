//! This file contains all the stuff about Fifo

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

/// This structure represents a FileOperation of type Fifo
#[derive(Debug, Default)]
pub struct Fifo {}

/// Main implementation for Fifo
impl Fifo {
    pub fn new() -> Self {
        Self {}
    }
}

/// Main Trait implementation
impl FileOperation for Fifo {
    fn register(&mut self, _access_mode: Mode) {
        unimplemented!();
    }
    fn unregister(&mut self, _access_mode: Mode) {
        unimplemented!();
    }
    fn read(&mut self, _buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
    fn write(&mut self, _buf: &[u8]) -> SysResult<IpcResult<u32>> {
        unimplemented!();
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Fifo {
    fn drop(&mut self) {
        println!("Fifo droped !");
    }
}
