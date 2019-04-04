use super::physical_page_allocator::{AllocFlags, PHYSICAL_ALLOCATOR};
use crate::memory::mmu::invalidate_page;
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
    pub fn reserve(&mut self, vaddr: Page<Virt>, paddr: Page<Phys>, size: NbrPages) -> Result<()> {
        //TODO: reserve the buddys
        unsafe {
            self.mmu.map_range_page(vaddr, paddr, size, Entry::READ_WRITE | Entry::PRESENT)?;
        }
        Ok(())
    }

    pub fn alloc(&mut self, size: NbrPages) -> Result<Page<Virt>> {
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;

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

    pub fn valloc(&mut self, size: NbrPages) -> Result<Page<Virt>> {
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;

        unsafe {
            self.mmu.map_range_page(vaddr, Page::new(0), size, Entry::READ_WRITE | Entry::VALLOC).map_err(|e| {
                self.virt.free(vaddr, order).unwrap();
                e
            })?;
        }
        Ok(vaddr)
    }

    pub fn valloc_handle_page_fault(&mut self, cr2: u32) -> Result<()> {
        let p = Page::containing(Virt(cr2 as usize));
        // TODO: remove this unwrap
        let entry = self.mmu.get_entry_mut(p).unwrap();
        if entry.contains(Entry::VALLOC) {
            let paddr = unsafe {
                PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(NbrPages(1), AllocFlags::KERNEL_MEMORY).map_err(|e| e)
            }?;
            entry.set_entry_page(paddr);
            *entry |= Entry::PRESENT;
            Ok(())
        } else {
            Err(MemoryError::PageFault)
        }
    }

    pub fn free(&mut self, vaddr: Page<Virt>, size: NbrPages) -> Result<()> {
        let order = size.into();
        self.virt.free(vaddr, order)?;

        if let Some(entry) = self.mmu.get_entry(vaddr) {
            if entry.contains(Entry::VALLOC) {
                // Free of Valloced memory
                for virtp in (vaddr..vaddr + size).iter() {
                    let entry = self.mmu.get_entry_mut(virtp).unwrap();
                    if entry.contains(Entry::PRESENT) {
                        unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap().free(entry.entry_page(), NbrPages(1)) }?;
                        invalidate_page(virtp);
                    }
                    *entry = Default::default();
                }
                Ok(())
            } else {
                // Free of Alloced memory
                unsafe {
                    PHYSICAL_ALLOCATOR.as_mut().unwrap().free(entry.entry_page(), size)?;
                    self.mmu.unmap_range_page(vaddr, size.into())
                }
            }
        } else {
            Err(MemoryError::NotPhysicallyMapped)
        }
    }
}

pub static mut KERNEL_VIRTUAL_PAGE_ALLOCATOR: Option<VirtualPageAllocator> = None;
