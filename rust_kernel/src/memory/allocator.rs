pub mod kernel;
pub use kernel::init_kernel_virtual_allocator;
pub use kernel::KERNEL_VIRTUAL_PAGE_ALLOCATOR;

mod physical;
pub use physical::{init_physical_allocator, PHYSICAL_ALLOCATOR};

mod r#virtual;
pub use r#virtual::VirtualPageAllocator;

mod slab;
use slab::SlabAllocator;

mod buddy;
use buddy::BuddyAllocator;
