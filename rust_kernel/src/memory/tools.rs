pub mod address;
pub use address::*;
pub mod nbr_pages;
pub use nbr_pages::*;
#[macro_use]
pub mod sections;
pub use sections::*;

pub mod device_map;
pub use device_map::*;

pub mod alloc_flags;
pub use alloc_flags::*;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// This might also significate that the allocator has no memory for internal storage of metadatas left.
    OutOfMem,
    OutOfBound,
    AlreadyOccupied,
    //    NotSatifiableFlags,
    AlreadyMapped,
    AlreadyUnMapped,
    CannotFree,
    NotPhysicallyMapped,
    PageFault,
    PageTableNotPresent,
    NotAllocated,
}

pub type Result<T> = core::result::Result<T, MemoryError>;
