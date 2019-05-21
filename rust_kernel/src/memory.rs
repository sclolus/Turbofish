//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
//! usize for a memory quantity is interpreted as a number a bytes

#[macro_use]
pub mod tools;

pub mod allocator;
pub use allocator::kernel::ffi::*;
pub use allocator::kernel::{get_physical_addr, mmap, munmap};

pub mod init;
pub use init::init_memory_system;

pub mod mmu;
