use super::buddy_allocator::BuddyAllocator;
use super::MemoryError;
use super::NbrPages;
use super::Order;
use super::PhysicalAddr;

/// 64 MB for the kernel memory
const KERNEL_PHYSIC_MEMORY: NbrPages = NbrPages::_32MB;

/// kernel memory start a 64 MB
//TODO: change that for the linker offset
const KERNEL_PHYSIC_OFFSET: usize = 0x4_000_000;

pub struct PhysicalAllocator {
    kernel_allocator: BuddyAllocator<PhysicalAddr>,
    _user_allocator: BuddyAllocator<PhysicalAddr>,
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalAllocator> = None;

impl PhysicalAllocator {
    pub fn new() -> Self {
        unsafe {
            Self {
                kernel_allocator: BuddyAllocator::new(KERNEL_PHYSIC_OFFSET, KERNEL_PHYSIC_MEMORY),
                _user_allocator: BuddyAllocator::new(
                    // offset in bytes
                    KERNEL_PHYSIC_OFFSET + Into::<usize>::into(KERNEL_PHYSIC_MEMORY),
                    // size in page
                    crate::multiboot::MULTIBOOT_INFO.unwrap().get_memory_amount_nb_pages()
                        - KERNEL_PHYSIC_OFFSET.into()
                        - KERNEL_PHYSIC_MEMORY,
                ),
            }
        }
    }
    /// size in bytes
    pub fn alloc_kernel(&mut self, size: usize) -> Result<PhysicalAddr, MemoryError> {
        self.kernel_allocator.alloc(Order::from_size(size))
    }
    /// size in bytes
    pub fn free_kernel(&mut self, addr: PhysicalAddr, size: usize) -> Result<(), MemoryError> {
        self.kernel_allocator.free(addr, Order::from_size(size))
    }
}

pub fn init_physical_allocator() {
    unsafe {
        PHYSICAL_ALLOCATOR = match PHYSICAL_ALLOCATOR {
            Some(_) => panic!("double init physical allocator"),
            None => Some(PhysicalAllocator::new()),
        }
    }
}
