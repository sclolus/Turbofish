use super::physical_page_allocator::{AllocFlags, PHYSICAL_ALLOCATOR};
use crate::memory::mmu::Entry;
use crate::memory::mmu::PageDirectory;
use crate::memory::tools::*;
use crate::memory::BuddyAllocator;
use alloc::boxed::Box;
/// A Physical Allocator must be registered to work
pub struct VirtualPageAllocator {
    virt: BuddyAllocator<Virt>,
    mmu: Box<PageDirectory>,
}

impl VirtualPageAllocator {
    pub fn new(virt: BuddyAllocator<Virt>, mmu: Box<PageDirectory>) -> Self {
        Self { virt, mmu }
    }
    /// size in bytes
    pub fn reserve(&mut self, vaddr: Page<Virt>, paddr: Page<Phys>, size: NbrPages) -> Result<()> {
        //TODO: reserve the buddys
        unsafe {
            self.mmu.map_range_page(vaddr, paddr, size, Entry::READ_WRITE | Entry::PRESENT)?;
        }
        Ok(())
    }

    /// size in bytes
    pub fn alloc(&mut self, size: NbrPages) -> Result<Page<Virt>> {
        //println!("alloc size: {:?}", size);
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        // let v: Virt = vaddr.into();
        // eprintln!("virtual alloc: {:x?}", v);
        unsafe {
            let paddr = PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(size, AllocFlags::KERNEL_MEMORY).map_err(|e| {
                self.virt.free(vaddr, order).unwrap();
                e
            })?;
            self.mmu.map_range_page(vaddr, paddr, size, Entry::READ_WRITE | Entry::PRESENT).map_err(|e| {
                self.virt.free(vaddr, order).unwrap();
                PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size).unwrap();
                e
            })?;
        }
        Ok(vaddr.into())
    }

    /// size in bytes
    pub fn free(&mut self, vaddr: Page<Virt>, size: NbrPages) -> Result<()> {
        //println!("free size: {:?}", size);
        let order = size.into();
        self.virt.free(vaddr, order)?;

        if let Some(paddr) = unsafe { self.mmu.physical_page(vaddr) } {
            unsafe {
                PHYSICAL_ALLOCATOR.as_mut().unwrap().free(paddr, size)?;
                self.mmu.unmap_range_page(vaddr, size.into())
            }
        } else {
            Err(MemoryError::NotPhysicalyMapped)
        }
    }
}

pub static mut KERNEL_VIRTUAL_PAGE_ALLOCATOR: Option<VirtualPageAllocator> = None;
