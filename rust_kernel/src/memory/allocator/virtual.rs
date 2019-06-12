use super::{BuddyAllocator, PHYSICAL_ALLOCATOR};
use crate::memory::mmu::{invalidate_page, Entry, PageDirectory};
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

impl VirtualPageAllocator {
    pub fn new(virt: BuddyAllocator<Virt>, mmu: Box<PageDirectory>) -> Self {
        Self { virt, mmu }
    }

    /// Modify the allocFlags for a specific and existing Page
    #[inline(always)]
    pub fn modify_page_entry(&mut self, page: Page<Virt>, flags: AllocFlags) {
        self.mmu.modify_page_entry(page, Into::<Entry>::into(flags));
    }

    /// Modify the AllocFlags of a given range of existing Virtual pages
    pub fn modify_range_page_entry(&mut self, start_page: Page<Virt>, nbr_pages: NbrPages, flags: AllocFlags) {
        for i in 0..nbr_pages.0 {
            self.modify_page_entry(start_page + NbrPages(i), flags);
        }
    }

    /// Check if the predicate is satisfied into a chunk of pages
    pub fn check_page_range<P>(&self, start_page: Page<Virt>, end_page: Page<Virt>, predicate: P) -> Result<()>
    where
        P: Fn(Entry) -> bool,
    {
        for page in (start_page..=end_page).iter() {
            if !predicate(self.mmu.get_entry(page).ok_or::<MemoryError>(MemoryError::PageNotPresent)?) {
                return Err(MemoryError::NotSatisfied);
            }
        }
        Ok(())
    }

    /// get the physical mapping of virtual address `v`
    pub unsafe fn get_physical_addr(&self, v: Virt) -> Option<Phys> {
        let offset = v.offset();
        self.mmu.get_entry(Page::containing(v)).map(|e| e.entry_addr() + offset)
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
            self.mmu.map_range_page(vaddr, paddr, size, Entry::READ_WRITE | Entry::PRESENT)?;
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
            self.mmu.map_range_page(vaddr, paddr, order.into(), Entry::READ_WRITE | Entry::PRESENT).map_err(|e| {
                self.virt.free(vaddr, order).expect("Could not free memory on VirtualPageAllocator");
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

    pub fn alloc_on(&mut self, vaddr: Page<Virt>, size: NbrPages, flags: AllocFlags) -> Result<Page<Virt>> {
        let order = size.into();
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        let entry = Entry::from(flags) | Entry::PRESENT;

        self.virt.reserve_exact(vaddr, order)?;
        unsafe {
            let paddr = physical_allocator.alloc(size, flags).map_err(|e| {
                self.virt
                    .free_reserve(vaddr, order)
                    .expect("Failed to free allocated virtual page after physical allocator failed");
                e
            })?;
            self.mmu.map_range_page(vaddr, paddr, order.into(), entry).map_err(|e| {
                self.virt.free_reserve(vaddr, order).expect("Could not free memory reserved on VirtualPageAllocator");
                physical_allocator.free(paddr).expect("Could not free memory on PhysicalAllocator");
                e
            })?;
        }
        Ok(vaddr.into())
    }

    pub fn alloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Virt>> {
        let order = size.into();
        let vaddr = self.virt.alloc(order)?;
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        let entry = Entry::from(flags) | Entry::PRESENT;

        unsafe {
            let paddr = physical_allocator.alloc(size, flags).map_err(|e| {
                self.virt
                    .free(vaddr, order)
                    .expect("Failed to free allocated virtual page after physical allocator failed");
                e
            })?;
            self.mmu.map_range_page(vaddr, paddr, order.into(), entry).map_err(|e| {
                self.virt.free(vaddr, order).expect("Could not free memory on VirtualPageAllocator");
                physical_allocator.free(paddr).expect("Could not free memory on PhysicalAllocator");
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
            self.mmu.map_range_page(vaddr, Page::new(0), order.into(), entry | Entry::VALLOC).map_err(|e| {
                self.virt.free(vaddr, order).expect("Failed to free virtual page after mapping failed");
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
            let paddr = physical_allocator.alloc(NbrPages(1), AllocFlags::KERNEL_MEMORY).map_err(|e| e)?;
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

    pub fn free(&mut self, vaddr: Page<Virt>) -> Result<()> {
        let size = self.ksize(vaddr)?;
        let order = size.into();
        let physical_allocator = unsafe { PHYSICAL_ALLOCATOR.as_mut().unwrap() };
        self.virt.free(vaddr, order)?;

        self.mmu.get_entry(vaddr).ok_or(MemoryError::NotPhysicallyMapped).and_then(|entry| {
            if entry.contains(Entry::VALLOC) {
                // Free of Valloced memory
                for virtp in (vaddr..vaddr + size).iter() {
                    let entry = self.mmu.get_entry_mut(virtp).expect("Could not find valloced page entry");
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

use core::mem::size_of;

#[derive(Debug)]
/// Virtual Allocator Specialized for processus
pub struct AddressSpace(VirtualPageAllocator);

impl AddressSpace {
    pub unsafe fn try_new() -> Result<Self> {
        let mut buddy = BuddyAllocator::new(Page::new(0x0), NbrPages::_3GB)?;
        buddy.reserve_exact(Page::new(0x0), NbrPages::_4MB).expect("User Buddy won't collaborate");

        let pd = PageDirectory::new_for_process()?;

        Ok(Self(VirtualPageAllocator::new(buddy, pd)))
    }

    /// the process forker must be the current cr3
    pub fn fork(&self) -> Result<Self> {
        let buddy = self.0.virt.try_clone().map_err(|_| MemoryError::OutOfMem)?;

        let pd = unsafe { self.0.mmu.fork()? };

        Ok(Self(VirtualPageAllocator::new(buddy, pd)))
    }

    /// Check if a pointer given by user process is not bullshit
    pub fn check_user_ptr<T>(&self, ptr: *const T) -> Result<()> {
        let start_ptr = Virt(ptr as usize);
        let end_ptr = Virt((ptr as usize).checked_add(size_of::<T>() - 1).ok_or(MemoryError::BadAddr)?);

        Ok(self
            .0
            .check_page_range(start_ptr.into(), end_ptr.into(), |entry| {
                entry.intersects((AllocFlags::USER_MEMORY).into())
            })
            .map_err(|_| MemoryError::BadAddr)?)
    }

    /// Check if a pointer given by user process is not bullshit
    pub fn check_user_ptr_with_len<T>(&self, ptr: *const T, length: usize) -> Result<()> {
        assert!(length != 0);
        let start_ptr = Virt(ptr as usize);
        let end_ptr = Virt((ptr as usize).checked_add(length - 1).ok_or(MemoryError::BadAddr)?);

        Ok(self
            .0
            .check_page_range(start_ptr.into(), end_ptr.into(), |entry| {
                entry.intersects((AllocFlags::USER_MEMORY).into())
            })
            .map_err(|_| MemoryError::BadAddr)?)
    }

    pub fn alloc<N>(&mut self, length: N, alloc_flags: AllocFlags) -> Result<*mut u8>
    where
        N: Into<NbrPages>,
    {
        Ok(self.0.alloc(length.into(), alloc_flags | AllocFlags::USER_MEMORY)?.to_addr().0 as *mut u8)
    }

    pub unsafe fn context_switch(&self) {
        self.0.context_switch()
    }

    pub fn modify_range_page_entry(&mut self, start_page: Page<Virt>, nbr_pages: NbrPages, flags: AllocFlags) {
        //TODO: check range in user_memory
        self.0.modify_range_page_entry(start_page, nbr_pages, flags | AllocFlags::USER_MEMORY);
    }

    pub fn alloc_on<N>(&mut self, vaddr: Page<Virt>, size: N, flags: AllocFlags) -> Result<*mut u8>
    where
        N: Into<NbrPages>,
    {
        Ok(self.0.alloc_on(vaddr, size.into(), flags | AllocFlags::USER_MEMORY)?.to_addr().0 as *mut u8)
    }

    #[inline(always)]
    pub fn modify_page_entry(&mut self, page: Page<Virt>, flags: AllocFlags) {
        //TODO: check range in user_memory
        self.0.modify_page_entry(page, flags | AllocFlags::USER_MEMORY);
    }
}
