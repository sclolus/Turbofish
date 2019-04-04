pub mod kernel_allocator;
pub use kernel_allocator::init_kernel_virtual_allocator;
pub use kernel_allocator::RustGlobalAlloc;

pub mod physical_page_allocator;
pub use physical_page_allocator::init_physical_allocator;

pub mod virtual_page_allocator;

pub mod slab_allocator;
pub use slab_allocator::SlabAllocator;

pub mod buddy_allocator;
pub use buddy_allocator::BuddyAllocator;

use crate::memory::tools::*;

/// 64 MB for the kernel memory
#[allow(dead_code)]
const KERNEL_PHYSICAL_MEMORY: NbrPages = NbrPages::_64MB;
#[allow(dead_code)]
// WTF ?!?
const KERNEL_VIRTUAL_MEMORY: NbrPages = NbrPages::_64MB;
