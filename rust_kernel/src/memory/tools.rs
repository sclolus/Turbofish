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

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// This might also significate that the allocator has no memory for internal storage of metadatas left.
    OutOfMem,
    /// Address parameter is out of bound
    OutOfBound,
    AlreadyOccupied,
    /// Address already mapped in mmu
    AlreadyMapped,
    /// Address already unmapped in mmu
    AlreadyUnMapped,
    CannotFree,
    NotPhysicallyMapped,
    /// Used to indicate the page fault handler that it is realy a page fault in vmalloc handle page fault
    PageFault,
    /// The specified page is marked as non-present
    PageNotPresent,
    PageTableNotPresent,
    NotAllocated,
    /// All conditions are not satisfied
    NotSatisfied,
}

pub type Result<T> = core::result::Result<T, MemoryError>;
