use super::buddy_allocator::BuddyAllocator;
use super::MemoryError;
use super::NbrPages;
use super::Order;
use super::PhysicalAddr;
use alloc::vec;

/// 64 MB for the kernel memory
const KERNEL_PHYSIC_MEMORY: NbrPages = NbrPages::_32MB;

/// kernel memory start a 64 MB
//TODO: change that for the linker offset
const KERNEL_PHYSIC_OFFSET: usize = 0x4_000_000;

/// 4 MB for the bootstrap
const MEMORY_BOOTSTRAP_ALLOCATOR: usize = 0x400_000;

pub static mut ALLOCATOR: Allocator = Allocator::Bootstrap(BootstrapAllocator::new());

pub enum Allocator {
    Bootstrap(BootstrapAllocator),
    Physical(PhysicalAllocator),
}

static mut BSS_MEMORY: [u8; MEMORY_BOOTSTRAP_ALLOCATOR] = [0; MEMORY_BOOTSTRAP_ALLOCATOR];
// TODO: align that

#[derive(Debug)]
pub struct BootstrapAllocator {
    current_offset: usize,
}

use core::fmt;

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Allocator::Bootstrap(b) => write!(f, "bootstrap allocator {:?}", b),
            Allocator::Physical(_p) => write!(f, "physical allocator"),
        }
    }
}

use core::alloc::Layout;

impl BootstrapAllocator {
    pub const fn new() -> Self {
        BootstrapAllocator { current_offset: 0 }
    }
    pub unsafe fn alloc_bootstrap(&mut self, size: usize, layout: Layout) -> Result<PhysicalAddr, MemoryError> {
        println!("{:?}", layout);
        println!("{:x?}", &BSS_MEMORY[0] as *const u8 as usize);
        println!("{:x?}", &BSS_MEMORY[self.current_offset] as *const u8 as usize);

        let address = &BSS_MEMORY[self.current_offset] as *const u8 as usize;
        self.current_offset += size;
        if self.current_offset > BSS_MEMORY.len() {
            panic!("No more bootstrap memory");
        }
        Ok(PhysicalAddr(address))
    }
}

#[derive(Debug)]
pub struct PhysicalAllocator {
    kernel_allocator: BuddyAllocator<PhysicalAddr>,
}

// pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalAllocator> = None;

impl PhysicalAllocator {
    pub fn new() -> Self {
        unsafe {
            Self {
                kernel_allocator: BuddyAllocator::new(
                    KERNEL_PHYSIC_OFFSET,
                    KERNEL_PHYSIC_MEMORY,
                    vec![0; BuddyAllocator::<PhysicalAddr>::metadata_from_nb_pages(KERNEL_PHYSIC_MEMORY)],
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
    let physical = PhysicalAllocator::new();
    unsafe {
        ALLOCATOR = Allocator::Physical(physical);
        dbg!(&ALLOCATOR as *const Allocator as *const u8);
    }
}
