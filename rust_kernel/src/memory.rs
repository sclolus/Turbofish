//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
//! usize for a memory quantity is interpreted as a number a bytes

#[macro_use]
pub mod tools;

pub mod allocator;
pub use allocator::kernel::ffi;
pub use allocator::kernel::{set_faillible_context, unset_faillible_context, RustGlobalAlloc};
pub use allocator::{VirtualPageAllocator, HIGH_KERNEL_MEMORY, KERNEL_VIRTUAL_PAGE_ALLOCATOR};

pub mod address_space;
pub use address_space::AddressSpace;

pub mod init;
pub use init::init_memory_system;

pub mod mmu;
