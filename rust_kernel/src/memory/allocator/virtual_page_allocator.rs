use super::physical_page_allocator::{AllocFlags, PHYSICAL_ALLOCATOR};
use crate::memory::mmu::PageDirectory;
use crate::memory::tools::*;
use crate::memory::BuddyAllocator;
use alloc::boxed::Box;
/// A Physical Allocator must be registered to work
pub struct VirtualPageAllocator {
    virt: BuddyAllocator<VirtualAddr>,
    mmu: Box<PageDirectory>,
}

impl VirtualPageAllocator {
    pub fn new(virt: BuddyAllocator<VirtualAddr>, mmu: Box<PageDirectory>) -> Self {
        unsafe { Self { virt, mmu } }
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
            self.mmu.map_range_page(Page::containing(vaddr), Page::containing(paddr), size.into()).map_err(|e| {
                self.virt.free(vaddr, order).unwrap();
                PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size).unwrap();
                e
            })?;
        }
        Ok(vaddr)
    }

    /// size in bytes
    pub fn free(&mut self, vaddr: VirtualAddr, size: usize) -> Result<(), MemoryError> {
        //println!("free size: {:?}", size);
        let order = size.into();
        self.virt.free(vaddr, order)?;

        if let Some(paddr) = unsafe { self.mmu.physical_addr(vaddr) } {
            unsafe {
                PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size)?;
                self.mmu.unmap_range_page(Page::containing(vaddr), size.into())
            }
        } else {
            Err(MemoryError::NotPhysicalyMapped)
        }
    }
}

pub static mut KERNEL_VIRTUAL_PAGE_ALLOCATOR: Option<VirtualPageAllocator> = None;
