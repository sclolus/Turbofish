use super::physical_allocator::{AllocFlags, PHYSICAL_ALLOCATOR};
use super::{KERNEL_VIRTUAL_MEMORY, KERNEL_VIRTUAL_OFFSET};
use crate::memory::mmu::PAGE_DIRECTORY;
use crate::memory::tools::*;
use crate::memory::BuddyAllocator;
use alloc::vec;
use core::alloc::{GlobalAlloc, Layout};
use core::fmt;

/// 4 MB for the bootstrap
const MEMORY_BOOTSTRAP_ALLOCATOR: usize = 0x400_000;

pub static mut ALLOCATOR: Allocator = Allocator::Bootstrap(BootstrapAllocator::new());

pub enum Allocator {
    Bootstrap(BootstrapAllocator),
    Kernel(KernelAllocator),
}

static mut BSS_MEMORY: [u8; MEMORY_BOOTSTRAP_ALLOCATOR] = [0; MEMORY_BOOTSTRAP_ALLOCATOR];

#[derive(Debug)]
pub struct BootstrapAllocator {
    current_offset: usize,
}

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Allocator::Bootstrap(b) => write!(f, "bootstrap allocator {:?}", b),
            Allocator::Kernel(_p) => write!(f, "physical allocator"),
        }
    }
}

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
/// A Physical Allocator must be registered to work
pub struct KernelAllocator {
    virt: BuddyAllocator<VirtualAddr>,
}

impl KernelAllocator {
    pub fn new() -> Self {
        unsafe {
            Self {
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
        //println!("alloc size: {:?}", size);
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        unsafe {
            let paddr = PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(size, AllocFlags::KERNEL_MEMORY).map_err(|e| {
                self.virt.free(vaddr, order).unwrap();
                e
            })?;
            PAGE_DIRECTORY.map_range_page(Page::containing(vaddr), Page::containing(paddr), size.into()).map_err(
                |e| {
                    self.virt.free(vaddr, order).unwrap();
                    PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size).unwrap();
                    e
                },
            )?;
        }
        Ok(vaddr)
    }

    /// size in bytes
    pub fn free(&mut self, vaddr: VirtualAddr, size: usize) -> Result<(), MemoryError> {
        //println!("free size: {:?}", size);
        let order = size.into();
        self.virt.free(vaddr, order)?;

        if let Some(paddr) = unsafe { PAGE_DIRECTORY.physical_addr(vaddr) } {
            unsafe {
                PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size)?;
                PAGE_DIRECTORY.unmap_range_page(Page::containing(vaddr), size.into())
            }
        } else {
            Err(MemoryError::NotPhysicalyMapped)
        }
    }
}

pub unsafe fn init_virtual_allocator() {
    let virt = KernelAllocator::new();
    ALLOCATOR = Allocator::Kernel(virt);
    dbg!(&ALLOCATOR as *const Allocator as *const u8);
}

pub struct MemoryManager;

unsafe impl GlobalAlloc for MemoryManager {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //core::ptr::null::<u8>() as *mut u8

        match &mut ALLOCATOR {
            Allocator::Kernel(a) => a.alloc(layout.size()).unwrap().0 as *mut u8, //.unwrap_or(PhysicalAddr(0x0)).0 as *mut u8
            Allocator::Bootstrap(b) => b.alloc_bootstrap(layout.size(), layout).unwrap().0 as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match &mut ALLOCATOR {
            Allocator::Kernel(a) => a.free(VirtualAddr(ptr as usize), layout.size()).unwrap(), //.unwrap_or(PhysicalAddr(0x0)).0 as *mut u8
            Allocator::Bootstrap(_) => panic!("try to free while in bootstrap allocator"),
        }
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}
