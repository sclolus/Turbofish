pub mod kernel_allocator;
pub use kernel_allocator::init_kernel_virtual_allocator;
pub use kernel_allocator::RustGlobalAlloc;

pub mod physical_page_allocator;
pub use physical_page_allocator::{init_physical_allocator, DeviceMap};

pub mod virtual_page_allocator;

pub mod slab_allocator;

pub mod buddy_allocator;
pub use buddy_allocator::BuddyAllocator;

use crate::memory::tools::*;

/// 64 MB for the kernel memory
const KERNEL_PHYSICAL_MEMORY: NbrPages = NbrPages::_64MB;
const KERNEL_VIRTUAL_MEMORY: NbrPages = NbrPages::_64MB;
