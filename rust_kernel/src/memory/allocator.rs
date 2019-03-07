pub mod kernel_allocator;
pub use kernel_allocator::init_kernel_virtual_allocator;
pub use kernel_allocator::MemoryManager;

pub mod physical_page_allocator;
pub use physical_page_allocator::init_physical_allocator;

pub mod virtual_page_allocator;

pub mod slab_allocator;

pub mod buddy_allocator;
pub use buddy_allocator::BuddyAllocator;

use crate::memory::tools::*;

/// 64 MB for the kernel memory
const KERNEL_PHYSICAL_MEMORY: NbrPages = NbrPages::_64MB;
const KERNEL_VIRTUAL_MEMORY: NbrPages = NbrPages::_64MB;

/// kernel memory start a 64 MB
//TODO: change that for the linker offset
const KERNEL_PHYSICAL_OFFSET: usize = 0x4_000_000;
const KERNEL_VIRTUAL_OFFSET: usize = 0x4_000_000;
