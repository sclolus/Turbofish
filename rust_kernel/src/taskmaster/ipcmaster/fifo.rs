//! This file contains all the stuff about Fifo

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
}

/// Some boilerplate to check if all is okay
impl Drop for Fifo {
    fn drop(&mut self) {
        println!("Fifo droped !");
    }
}
