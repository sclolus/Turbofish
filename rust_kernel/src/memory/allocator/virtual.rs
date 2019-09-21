use super::{BuddyAllocator, PHYSICAL_ALLOCATOR};
use crate::memory::mmu::{_read_cr3, invalidate_page, invalidate_page_range, Entry, PageDirectory};
use crate::memory::tools::*;
use alloc::boxed::Box;
use core::convert::Into;
use fallible_collections::TryClone;

/// A Physical Allocator must be registered to work
pub struct VirtualPageAllocator {
    virt: BuddyAllocator<Virt>,
    mmu: Box<PageDirectory>,
}

use core::{fmt, fmt::Debug};
impl Debug for VirtualPageAllocator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Virtual allocator")
    }
}

/// Drop protector
impl Drop for VirtualPageAllocator {
    fn drop(&mut self) {
        if unsafe {
            self.get_physical_addr(Virt(self.mmu.as_ref() as *const _ as usize))
                .expect("Woot ?")
                == _read_cr3()
        } {
            panic!("Page Directory Auto-Sodomization: Would you really trash your current CR3 ?");
        }
    }
}

impl VirtualPageAllocator {
    pub fn new(virt: BuddyAllocator<Virt>, mmu: Box<PageDirectory>) -> Self {
        Self { virt, mmu }
    }

    /// Just for the handled PageDirectory
    pub unsafe fn fork_pd(&self) -> Result<Box<PageDirectory>> {
        self.mmu.fork()
    }

    /// Fork the VirtualPageAllocator
    pub fn fork(&self) -> Result<Self> {
        let buddy = self.virt.try_clone().map_err(|_| MemoryError::OutOfMem)?;

        let pd = unsafe { self.fork_pd()? };

        Ok(VirtualPageAllocator::new(buddy, pd))
    }

    /// Modify the allocFlags for a specific and existing Page
    #[inline(always)]
    pub fn change_flags_page_entry(&mut self, page: Page<Virt>, flags: AllocFlags) {
        self.change_flags_range_page_entry(page, NbrPages(1), flags)
    }

    /// Modify the AllocFlags of a given range of existing Virtual pages
    pub fn change_flags_range_page_entry(
        &mut self,
        start_page: Page<Virt>,
        nbr_pages: NbrPages,
        flags: AllocFlags,
    ) {
        for i in 0..nbr_pages.0 {
            self.mmu
                .modify_page_entry(start_page + NbrPages(i), Into::<Entry>::into(flags));
        }
        invalidate_page_range(start_page, nbr_pages);
    }

    pub fn change_page_entry<U>(&mut self, page: Page<Virt>, update: &mut U) -> Result<()>
    where
        U: FnMut(&mut Entry),
    {
        self.change_range_page_entry(page, NbrPages(1), update)
    }

    pub fn change_range_page_entry<U>(
        &mut self,
        start_page: Page<Virt>,
        nbr_pages: NbrPages,
        update: &mut U,
    ) -> Result<()>
    where
        U: FnMut(&mut Entry),
    {
        Ok({
            for i in 0..nbr_pages.0 {
                update(
                    self.mmu
                        .get_entry_mut(start_page + NbrPages(i))
                        .ok_or::<MemoryError>(MemoryError::PageNotPresent)?,
                );
            }
            invalidate_page_range(start_page, nbr_pages)
        })
    }

    /// Check if the predicate is satisfied into a chunk of pages
    pub fn check_page_range<P>(
        &self,
        start_page: Page<Virt>,
        end_page: Page<Virt>,
        predicate: P,
    ) -> Result<()>
    where
        P: Fn(Entry) -> bool,
    {
        for page in (start_page..=end_page).iter() {
            if !predicate(
                self.mmu
                    .get_entry(page)
                    .ok_or::<MemoryError>(MemoryError::PageNotPresent)?,
            ) {
                return Err(MemoryError::NotSatisfied);
            }
        }
        Ok(())
    }

    /// get the physical mapping of virtual address `v`
    pub unsafe fn get_physical_addr(&self, v: Virt) -> Option<Phys> {
        let offset = v.offset();
        self.mmu
            .get_entry(Page::containing(v))
            .map(|e| e.entry_addr() + offset)
    }

    pub unsafe fn context_switch(&self) {
        PageDirectory::context_switch(&self.mmu);
    }

    // Should this have an AllocFlags.
    pub fn reserve(&mut self, vaddr: Page<Virt>, paddr: Page<Phys>, size: NbrPages) -> Result<()> {
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };

        unsafe {
            self.virt.reserve_exact(vaddr, size.into())?;
            let res = physical_allocator.reserve(paddr, size.into());

            match res {
                Ok(_) => (),
                Err(MemoryError::OutOfBound) => (), //Todo fix this eventually ?
                Err(e) => {
                    self.virt
                        .free_reserve(vaddr, size.into())
                        .expect("Could not free memory reserved on VirtualPageAllocator");
                    return Err(e);
                }
            }
            self.mmu
                .map_range_page(vaddr, paddr, size, Entry::READ_WRITE | Entry::PRESENT)?;
        }
        Ok(())
    }

    /// Map a ranged physical area and return a virtual address associated
    /// notice: fn(Phys(physical_address_to_map).into(), size.into()) -> Some stuff
    pub fn map_addr(&mut self, paddr: Page<Phys>, size: NbrPages) -> Result<Page<Virt>> {
        let order = size.into();

        // get a new chunk on kernel virtual buddy
        let vaddr = self.virt.alloc(order)?;

        unsafe {
            // map this virtual chunk with the associated physical address
            self.mmu
                .map_range_page(
                    vaddr,
                    paddr,
                    order.into(),
                    Entry::READ_WRITE | Entry::PRESENT,
                )
                .map_err(|e| {
                    self.virt
                        .free(vaddr, order)
                        .expect("Could not free memory on VirtualPageAllocator");
                    e
                })?;
        }
        Ok(vaddr)
    }

    /// UnMap a ranged virtual area
    /// notice: fn(Virt(physical_address_to_map).into()) -> Result
    pub fn unmap_addr(&mut self, vaddr: Page<Virt>, size: NbrPages) -> Result<()> {
        let order = size.into();

        // release the chunk on kernel virtual buddy
        self.virt.free(vaddr, order)?;

        // unmap this vitual chunk
        unsafe { self.mmu.unmap_range_page(vaddr, order.into()) }
    }

    pub fn alloc_on(
        &mut self,
        vaddr: Page<Virt>,
        size: NbrPages,
        flags: AllocFlags,
    ) -> Result<Page<Virt>> {
        let order = size.into();
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        let entry = Entry::from(flags) | Entry::PRESENT;

        self.virt.reserve_exact(vaddr, order)?;
        unsafe {
            let paddr = physical_allocator.alloc(size, flags).map_err(|e| {
                self.virt.free_reserve(vaddr, order).expect(
                    "Failed to free allocated virtual page after physical allocator failed",
                );
                e
            })?;
            self.mmu
                .map_range_page(vaddr, paddr, order.into(), entry)
                .map_err(|e| {
                    self.virt
                        .free_reserve(vaddr, order)
                        .expect("Could not free memory reserved on VirtualPageAllocator");
                    physical_allocator
                        .free(paddr)
                        .expect("Could not free memory on PhysicalAllocator");
                    e
                })?;
        }
        Ok(vaddr.into())
    }

    pub fn dealloc_on(&mut self, vaddr: Page<Virt>, size: NbrPages) -> Result<()> {
        let order = size.into();

        let page_paddr = unsafe {
            self.mmu
                .physical_page(vaddr)
                .ok_or(MemoryError::NotPhysicallyMapped)?
        };
        // release the chunk on kernel virtual buddy
        self.virt.free_reserve(vaddr, order)?;

        // Free the chunk on physical allocator
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        physical_allocator
            .free(page_paddr.into())
            .expect("never allocated");

        // unmap this vitual chunk
        unsafe { self.mmu.unmap_range_page(vaddr, order.into()) }
    }

    pub fn alloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Virt>> {
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        let entry = Entry::from(flags) | Entry::PRESENT;

        unsafe {
            let paddr = physical_allocator.alloc(size, flags).map_err(|e| {
                self.virt.free(vaddr, order).expect(
                    "Failed to free allocated virtual page after physical allocator failed",
                );
                e
            })?;
            self.mmu
                .map_range_page(vaddr, paddr, order.into(), entry)
                .map_err(|e| {
                    self.virt
                        .free(vaddr, order)
                        .expect("Could not free memory on VirtualPageAllocator");
                    physical_allocator
                        .free(paddr)
                        .expect("Could not free memory on PhysicalAllocator");
                    e
                })?;
        }
        Ok(vaddr.into())
    }

    pub fn valloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Virt>> {
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        let entry = Entry::from(flags);

        unsafe {
            self.mmu
                .map_range_page(vaddr, Page::new(0), order.into(), entry | Entry::VALLOC)
                .map_err(|e| {
                    self.virt
                        .free(vaddr, order)
                        .expect("Failed to free virtual page after mapping failed");
                    e
                })?;
        }
        Ok(vaddr)
    }

    pub fn valloc_handle_page_fault(&mut self, cr2: u32) -> Result<()> {
        let p = Page::containing(Virt(cr2 as usize));
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        let entry = self.mmu.get_entry_mut(p).ok_or(MemoryError::PageFault)?;

        if entry.contains(Entry::VALLOC) {
            // KERNEL_MEMORY flags is currently not used.
            let paddr = physical_allocator
                .alloc(NbrPages(1), AllocFlags::KERNEL_MEMORY)
                .map_err(|e| e)?;
            entry.set_entry_page(paddr);
            *entry |= Entry::PRESENT;
            Ok(())
        } else {
            Err(MemoryError::PageFault)
        }
    }

    pub fn ksize(&mut self, vaddr: Page<Virt>) -> Result<NbrPages> {
        Ok(self.virt.ksize(vaddr)?.nbr_pages())
    }

    // TODO: Fix that function !
    pub fn free(&mut self, vaddr: Page<Virt>) -> Result<()> {
        // The NbrPages returned is wrong !
        let size = self.ksize(vaddr)?;
        let order = size.into();
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        self.virt.free(vaddr, order)?;

        self.mmu
            .get_entry(vaddr)
            .ok_or(MemoryError::NotPhysicallyMapped)
            .and_then(|entry| {
                if entry.contains(Entry::VALLOC) {
                    // Free of Valloced memory
                    for virtp in (vaddr..vaddr + size).iter() {
                        let entry = self
                            .mmu
                            .get_entry_mut(virtp)
                            .expect("Could not find valloced page entry");
                        if entry.contains(Entry::PRESENT) {
                            physical_allocator.free(entry.entry_page())?;
                            invalidate_page(virtp);
                        }
                        *entry = Default::default();
                    }
                } else {
                    // Free of Alloced memory
                    physical_allocator.free(entry.entry_page())?;
                    unsafe { self.mmu.unmap_range_page(vaddr, size)? }
                }
                Ok(())
            })
    }
}
