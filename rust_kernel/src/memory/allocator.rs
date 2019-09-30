pub mod kernel;
pub use kernel::init_kernel_virtual_allocator;
pub use kernel::{HIGH_KERNEL_MEMORY, KERNEL_VIRTUAL_PAGE_ALLOCATOR};

mod physical;
pub use physical::{init_physical_allocator, PHYSICAL_ALLOCATOR};

mod r#virtual;
pub use r#virtual::VirtualPageAllocator;

mod buddy;
pub use buddy::BuddyAllocator;
