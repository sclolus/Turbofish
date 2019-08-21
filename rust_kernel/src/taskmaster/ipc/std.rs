//! This file contains all the stuff about std File Descriptors

use super::SysResult;

use super::Driver;
use super::FileOperation;
use super::IpcResult;
use super::Mode;

mod stdin;
pub use stdin::Stdin;
mod stdout;
pub use stdout::Stdout;
mod stderr;
pub use stderr::Stderr;

use alloc::sync::Arc;
use sync::dead_mutex::DeadMutex;

use crate::terminal::{ReadResult, TERMINAL};

/// This structure represents a FileOperation of type Std
#[derive(Debug, Default)]
pub struct Std {
    controlling_terminal: usize,
}

/// Main implementation for Std
impl Std {
    pub fn new(controlling_terminal: usize) -> Self {
        Self {
            controlling_terminal,
        }
    }
}

/// Main Trait implementation
impl FileOperation for Std {
    fn register(&mut self, _access_mode: Mode) {}
    fn unregister(&mut self, _access_mode: Mode) {}
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
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        print_tty!(self.controlling_terminal, "{}", unsafe {
            core::str::from_utf8_unchecked(buf)
        });
        Ok(IpcResult::Done(buf.len() as _))
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Std {
    fn drop(&mut self) {
        println!("Std file operation droped !");
    }
}

#[derive(Debug)]
pub struct Tty {
    inode_id: Option<usize>,
    operation: Arc<DeadMutex<Std>>,
}

impl Tty {
    pub fn new(operation: Arc<DeadMutex<Std>>) -> Self {
        Self {
            inode_id: None,
            operation,
        }
    }
}

impl Driver for Tty {
    fn open(&mut self) -> Arc<DeadMutex<dyn FileOperation>> {
        self.operation.clone()
    }
    fn set_inode_id(&mut self, inode_id: usize) {
        self.inode_id = Some(inode_id);
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Tty {
    fn drop(&mut self) {
        println!("tty driver droped !");
    }
}
