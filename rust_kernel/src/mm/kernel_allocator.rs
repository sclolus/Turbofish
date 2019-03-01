use super::buddy_allocator::BuddyAllocator;
use super::MemoryError;
use super::NbrPages;
use super::PAGE_DIRECTORY;
use super::{PhysicalAddr, VirtualAddr};
use alloc::vec;

/// 64 MB for the kernel memory
const KERNEL_PHYSICAL_MEMORY: NbrPages = NbrPages::_64MB;
const KERNEL_VIRTUAL_MEMORY: NbrPages = NbrPages::_64MB;

/// kernel memory start a 64 MB
//TODO: change that for the linker offset
const KERNEL_PHYSICAL_OFFSET: usize = 0x4_000_000;
const KERNEL_VIRTUAL_OFFSET: usize = 0x4_000_000;

/// 4 MB for the bootstrap
const MEMORY_BOOTSTRAP_ALLOCATOR: usize = 0x400_000;

pub static mut ALLOCATOR: Allocator = Allocator::Bootstrap(BootstrapAllocator::new());

pub enum Allocator {
    Bootstrap(BootstrapAllocator),
    Kernel(KernelAllocator),
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
            Allocator::Kernel(_p) => write!(f, "physical allocator"),
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
pub struct KernelAllocator {
    phys: BuddyAllocator<PhysicalAddr>,
    virt: BuddyAllocator<VirtualAddr>,
}

impl KernelAllocator {
    pub fn new() -> Self {
        unsafe {
            Self {
                phys: BuddyAllocator::new(
                    KERNEL_PHYSICAL_OFFSET,
                    KERNEL_PHYSICAL_MEMORY,
                    vec![0; BuddyAllocator::<PhysicalAddr>::metadata_size(KERNEL_PHYSICAL_MEMORY)],
                ),
                virt: BuddyAllocator::new(
                    KERNEL_VIRTUAL_OFFSET,
                    KERNEL_VIRTUAL_MEMORY,
                    vec![0; BuddyAllocator::<VirtualAddr>::metadata_size(KERNEL_VIRTUAL_MEMORY)],
                ),
            }
        }
    }
    /// size in bytes
    pub fn alloc(&mut self, size: usize) -> Result<VirtualAddr, MemoryError> {
        println!("alloc size: {:?}", size);
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        let paddr = self.phys.alloc(order).map_err(|e| {
            self.virt.free(vaddr, order).unwrap();
            e
        })?;
        unsafe {
            PAGE_DIRECTORY.map_range_addr(vaddr, paddr, size.into()).map_err(|_e| {
                self.virt.free(vaddr, order).unwrap();
                self.phys.free(paddr, order).unwrap();
                MemoryError::AlreadyOccupied
            })?;
        }
        Ok(vaddr)
    }

    /// size in bytes
    pub fn free(&mut self, addr: VirtualAddr, size: usize) -> Result<(), MemoryError> {
        println!("free size: {:?}", size);
        let order = size.into();
        self.virt.free(addr, order)?;
        if let Some(phys_addr) = addr.physical_addr() {
            self.phys.free(phys_addr, size.into())?;
        }
        Ok(())
    }
}

pub fn init_physical_allocator() {
    let physical = KernelAllocator::new();
    unsafe {
        ALLOCATOR = Allocator::Kernel(physical);
        dbg!(&ALLOCATOR as *const Allocator as *const u8);
    }
}
