use super::BuddyAllocator;
use super::{KERNEL_PHYSICAL_MEMORY, KERNEL_PHYSICAL_OFFSET};
use crate::memory::tools::*;
use alloc::vec;
use bitflags::bitflags;

#[derive(Debug)]
pub struct PhysicalPageAllocator {
    allocator: BuddyAllocator<PhysicalAddr>,
}

bitflags! {
    pub struct AllocFlags: u32 {
        const KERNEL_MEMORY = 1 << 0;
        const USER_MEMORY = 1 << 0;
    }
}

impl PhysicalPageAllocator {
    pub fn new() -> Self {
        unsafe {
            Self {
                allocator: BuddyAllocator::new(
                    KERNEL_PHYSICAL_OFFSET,
                    KERNEL_PHYSICAL_MEMORY,
                    vec![0; BuddyAllocator::<PhysicalAddr>::metadata_size(KERNEL_PHYSICAL_MEMORY)],
                ),
            }
        }
    }
    /// size in bytes
    pub fn alloc(&mut self, size: usize, flags: AllocFlags) -> Result<PhysicalAddr, MemoryError> {
        //println!("alloc size: {:?}", size);
        if flags.contains(AllocFlags::KERNEL_MEMORY) {
            let order = size.into();
            Ok(self.allocator.alloc(order)?)
        } else {
            unimplemented!()
        }
    }

    /// size in bytes
    pub fn free(&mut self, paddr: PhysicalAddr, size: usize) -> Result<(), MemoryError> {
        //println!("free size: {:?}", size);
        let order = size.into();
        Ok(self.allocator.free(paddr, order)?)
    }
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalPageAllocator> = None;

pub unsafe fn init_physical_allocator() {
    PHYSICAL_ALLOCATOR = Some(PhysicalPageAllocator::new());
}
