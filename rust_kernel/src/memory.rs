//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
//! usize for a memory quantity is interpreted as a number a bytes

pub const VIRTUAL_OFFSET: usize = 0xc0000000;

#[macro_use]
pub mod tools;
pub use tools::nbr_pages::NbrPages;

pub mod allocator;
pub use allocator::*;

pub mod init;
pub use init::init_memory_system;

mod mmu;