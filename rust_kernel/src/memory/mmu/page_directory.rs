//! This module contains the code related to the page directory and its page directory entries, which are the highest abstraction paging-related data structures (for the cpu)
//! See https://wiki.osdev.org/Paging for relevant documentation.
use super::page_table::PageTable;
use super::{Entry, _enable_paging, BIOS_PAGE_TABLE, PAGE_TABLES};
use crate::memory::allocator::{KERNEL_VIRTUAL_PAGE_ALLOCATOR, PHYSICAL_ALLOCATOR};
use crate::memory::tools::*;
use alloc::boxed::Box;
use core::mem::size_of;
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

/// This is the representation of the topmost paging structure.
/// It is composed of 1024 Entry.
/// This structure must be 4-KiB aligned.
#[repr(C, align(4096))]
pub struct PageDirectory {
    entries: [Entry; 1024],
}

impl PageDirectory {
    /// This fonction creates a PageDirectory at addr `page_directory_addr` of size (in elements) of `size`.
    pub const fn new() -> Self {
        Self { entries: [Entry::new(); 1024] }
    }

    /// create a new page directory for a process ( share all pages table above 3GB and the 1 page table with the kernel )
    pub fn new_for_process() -> Box<Self> {
        // map the kenel pages tables
        let mut pd = Box::new(Self::new());
        unsafe {
            pd.set_page_tables(0, &BIOS_PAGE_TABLE);
            pd.set_page_tables(768, &PAGE_TABLES);

            // get the physical addr of the page directory for the tricks
            let phys_pd: Phys = {
                let raw_pd = pd.as_mut() as *mut PageDirectory;
                KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().get_physical_addr(Virt(raw_pd as usize)).unwrap()
            };

            pd.self_map_tricks(phys_pd);
        }
        pd
    }

    pub unsafe fn context_switch(&self) {
        let phys_pd = {
            let raw_pd = self as *const Self;
            KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().get_physical_addr(Virt(raw_pd as usize)).unwrap()
        };
        _enable_paging(phys_pd);
    }

    // dummy fork for the moment ( no copy on write and a lot of context switch )
    pub unsafe fn fork(&self) -> Result<Box<Self>> {
        let mut mem_tmp = [0; PAGE_SIZE];
        let mut child = Self::new_for_process();

        // parcour the user page directory
        for i in 1..768 {
            let page = Page::new(i * 1024);
            if self[i].contains(Entry::PRESENT) {
                let page_table = self.get_page_table_trick(page).expect("can't happen");

                // parcour the user page table
                for j in 0..1024 {
                    let entry = page_table[j];
                    if entry.contains(Entry::PRESENT) {
                        // get the memory
                        let virt = page + NbrPages(j);
                        let mem = virt.to_addr().0 as *mut [u8; PAGE_SIZE];
                        mem_tmp = *mem;

                        child.as_ref().context_switch();
                        let phys =
                            PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(PAGE_SIZE.into(), AllocFlags::USER_MEMORY)?;
                        child.map_page(virt, phys, entry)?;
                        *(virt.to_addr().0 as *mut [u8; PAGE_SIZE]) = mem_tmp;
                        self.context_switch();
                    }
                }
            }
        }
        Ok(child)
    }

    /// This is a trick that ensures that the page tables are mapped into virtual memory at address 0xFFC00000 .
    /// The idea is that the last Entry points to self, viewed as a Page Table.
    /// See [Osdev](https://wiki.osdev.org/Memory_Management_Unit)
    pub unsafe fn self_map_tricks(&mut self, cr3: Phys) {
        self[1023] = Default::default();
        self[1023].set_entry_addr(cr3);
        self[1023] |= Entry::PRESENT | Entry::READ_WRITE;
    }

    /// set the page tables by translating the addresses of the slice by virtual_offset
    pub fn set_page_tables(&mut self, offset: usize, page_tables: &[PageTable]) {
        for (i, pt) in page_tables.iter().enumerate() {
            self[offset + i] = Default::default();
            self[offset + i].set_entry_addr(Phys(pt.as_ref().as_ptr() as usize - symbol_addr!(virtual_offset)));
            self[offset + i] |= Entry::PRESENT | Entry::READ_WRITE;
        }
    }

    /// get the page table without using the trick (by translating the addresses by virtual_offset)
    #[inline(always)]
    fn get_page_table_init(&self, virtp: Page<Virt>) -> Option<&mut PageTable> {
        let pd_index = virtp.pd_index();
        if !self[pd_index].contains(Entry::PRESENT) {
            return None;
        }
        // assert!(self as *const _ == 0xFFC00000 as *const _);
        Some(unsafe { &mut *((self[pd_index].entry_addr().0 + symbol_addr!(virtual_offset)) as *mut PageTable) })
    }

    /// get the page table corresponding to `virtp` using the trick
    #[inline(always)]
    fn get_page_table_trick(&self, virtp: Page<Virt>) -> Option<&mut PageTable> {
        let pd_index = virtp.pd_index();
        if !self[pd_index].contains(Entry::PRESENT) {
            return None;
        }
        Some(unsafe { &mut *((0xFFC00000 + pd_index * 4096) as *mut PageTable) })
    }

    /// get the page table corresponding to `virtp` using the trick
    /// and allocate a new one with the physical allocator if not present
    #[inline(always)]
    unsafe fn get_page_table_trick_alloc(&mut self, virtp: Page<Virt>) -> Result<&mut PageTable> {
        let pd_index = virtp.pd_index();
        if !self[pd_index].contains(Entry::PRESENT) {
            let new_page_table =
                PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(size_of::<PageTable>().into(), AllocFlags::KERNEL_MEMORY)?;
            self[pd_index].set_entry_page(new_page_table);
            self[pd_index] |= Entry::PRESENT | Entry::READ_WRITE;

            let slice =
                core::slice::from_raw_parts_mut((0xFFC00000 + pd_index * 4096) as *mut u8, size_of::<PageTable>());
            for i in slice {
                *i = 0;
            }
        }
        Ok(&mut *((0xFFC00000 + pd_index * 4096) as *mut PageTable))
    }

    /// get mutably the entry of the page table corresponding to `virtp`
    #[inline(always)]
    pub fn get_entry_mut(&mut self, virtp: Page<Virt>) -> Option<&mut Entry> {
        self.get_page_table_trick(virtp).map(|page_table| &mut page_table[virtp.pt_index()])
    }

    /// get the entry of the page table corresponding to `virtp`
    #[inline(always)]
    pub fn get_entry(&self, virtp: Page<Virt>) -> Option<Entry> {
        self.get_page_table_trick(virtp).map(|page_table| page_table[virtp.pt_index()])
    }

    /// use the self referencing trick. so must be called when paging is enabled and after self_map_tricks has been called
    #[inline(always)]
    pub unsafe fn map_page(&mut self, virtp: Page<Virt>, physp: Page<Phys>, entry: Entry) -> Result<()> {
        // We can't have hybrid permisions inside a page table.
        // So if we try to map a map as an User page, we need to set it to for the corresponding page directory entry.
        if entry.contains(Entry::USER) {
            self[virtp.pd_index()] |= Entry::USER;
        }
        self.get_page_table_trick_alloc(virtp)?.map_page(virtp, physp, entry)
    }

    //TODO: check overflow
    /// map the virutal pages (virtp..virtp + nb_pages) on the physical pages (physp..physp + nb_pages)
    pub unsafe fn map_range_page(
        &mut self,
        virtp: Page<Virt>,
        physp: Page<Phys>,
        nb_pages: NbrPages,
        entry: Entry,
    ) -> Result<()> {
        for (virtp, physp) in (virtp..virtp + nb_pages).iter().zip((physp..physp + nb_pages).iter()) {
            self.map_page(virtp, physp, entry)?;
        }
        Ok(())
    }

    /// map the virutal pages (virtp..virtp + nb_pages) on the physical pages (physp..physp + nb_pages)
    /// without using the trick. So it can be used in initialisation before enable paging
    pub unsafe fn map_range_page_init(
        &mut self,
        virtp: Page<Virt>,
        physp: Page<Phys>,
        nb_pages: NbrPages,
        entry: Entry,
    ) -> Result<()> {
        for (virtp, physp) in (virtp..virtp + nb_pages).iter().zip((physp..physp + nb_pages).iter()) {
            self.get_page_table_init(virtp)
                .ok_or(MemoryError::PageTableNotPresent)
                .and_then(|page_table| page_table.map_page(virtp, physp, entry))?
        }
        Ok(())
    }

    pub unsafe fn unmap_page(&mut self, virtp: Page<Virt>) -> Result<()> {
        self.get_page_table_trick(virtp)
            .ok_or(MemoryError::PageTableNotPresent)
            .and_then(|page_table| page_table.unmap_page(virtp))
    }

    //TODO: check overflow
    /// unmap the virutal pages (virtp..virtp + nb_pages) on the physical pages (physp..physp + nb_pages)
    pub unsafe fn unmap_range_page(&mut self, virtp: Page<Virt>, nb_pages: NbrPages) -> Result<()> {
        for p in (virtp..virtp + nb_pages).iter() {
            self.unmap_page(p)?;
        }
        Ok(())
    }

    /// get the physical page wich is mapped on `vaddr`
    pub unsafe fn physical_page(&self, vaddr: Page<Virt>) -> Option<Page<Phys>> {
        self.get_entry(vaddr).map(|entry| entry.entry_page())
    }

    /// get the physical address wich is mapped on `vaddr`
    pub unsafe fn physical_addr(&self, vaddr: Virt) -> Option<Phys> {
        self.physical_page(vaddr.into()).map(|v| v.into())
    }
}

/// The PageDirectory implements Index which enables us to use the syntax: `pd[index]`,
/// instead of `pd.entries[index]` in an immutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageDirectory, that is, if index >= PageDirectory.entries.len()
impl<'a, T> Index<T> for PageDirectory
where
    T: SliceIndex<[Entry]>,
{
    type Output = T::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        idx.index(&self.entries)
    }
}

/// The PageDirectory implements IndexMut which enables us to use the syntax: `pd[index] = SomeEntry`
/// instead of `pd.entries[index] = SomeEntry` in a mutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageDirectory, that is, if index >= PageDirectory.entries.len()
impl<'a, T> IndexMut<T> for PageDirectory
where
    T: SliceIndex<[Entry]>,
{
    #[inline]
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        idx.index_mut(&mut self.entries)
    }
}

impl AsRef<[Entry]> for PageDirectory {
    fn as_ref(&self) -> &[Entry] {
        &self.entries
    }
}

impl AsMut<[Entry]> for PageDirectory {
    fn as_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }
}

/// call the physical allocator to free the user page tables
impl Drop for PageDirectory {
    fn drop(&mut self) {
        for i in 1..768 {
            if self[i].contains(Entry::PRESENT) {
                unsafe {
                    //TODO: invalid page ?
                    PHYSICAL_ALLOCATOR.as_mut().unwrap().free(self[i].entry_page()).unwrap();
                }
            }
        }
    }
}
