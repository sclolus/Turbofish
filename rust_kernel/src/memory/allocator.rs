pub mod kernel_allocator;
pub use kernel_allocator::init_kernel_virtual_allocator;
pub use kernel_allocator::{kfree, kmalloc, ksize, vfree, vmalloc, vsize, RustGlobalAlloc};

pub mod physical_page_allocator;
pub use physical_page_allocator::{init_physical_allocator, PHYSICAL_ALLOCATOR};

pub mod virtual_page_allocator;

pub mod slab_allocator;
pub use slab_allocator::SlabAllocator;

pub mod buddy_allocator;
pub use buddy_allocator::BuddyAllocator;
