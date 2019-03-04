use super::buddy_allocator::BuddyAllocator;
use super::NbrPages;
use super::{Address, MemoryError, Page, PageIter, PAGE_SIZE};
use super::{PhysicalAddr, VirtualAddr};

/// 64 MB for the kernel memory
const KERNEL_REAL_SIZE: NbrPages = NbrPages::_64MB;

/// kernel memory start a 64 MB
//TODO: change that for the linker offset
const KERNEL_REAL_OFFSET: usize = 0x4_000_000;

pub struct PhysicalAllocator {
    kernel_alloc: BuddyAllocator<PhysicalAddr>,
    user_alloc: BuddyAllocator<PhysicalAddr>,
}

impl PhysicalAllocator {
    pub fn new() -> Self {
        Self {
            kernel_alloc: BuddyAllocator::new(KERNEL_REAL_OFFSET, KERNEL_REAL_SIZE),
            user_alloc: BuddyAllocator::new(
                // offset in bytes
                KERNEL_REAL_OFFSET + KERNEL_REAL_SIZE.into(),
                // size in page
                crate::multiboot::MULTIBOOT_INFO.unwrap().get_memory_amount_nb_pages()
                    - KERNEL_REAL_OFFSET.into()
                    - KERNEL_REAL_SIZE,
            ),
        }
    }
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalAllocator> = None;

pub fn init_physical_allocator() {
    PHYSICAL_ALLOCATOR = match PHYSICAL_ALLOCATOR {
        Some(_) => panic!("double init physical allocator"),
        None => Some(PhysicalAllocator::new()),
    }
}
