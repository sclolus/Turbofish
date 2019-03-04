//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
//! usize for a memory quantity is interpreted as a number a bytes
mod buddy_allocator;
use buddy_allocator::BuddyAllocator;

mod tools;
pub use tools::nbr_pages::NbrPages;
use tools::*;

mod kernel_allocator;
pub use kernel_allocator::MemoryManager;

pub mod init;
pub use init::init_memory_system;

mod mmu;
