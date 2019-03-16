//! This module contains the code related to the page directory and its page directory entries, which are the highest abstraction paging-related data structures (for the cpu)
//! See https://wiki.osdev.org/Paging for relevant documentation.
use super::page_table::PageTable;
use super::Entry;
use super::PAGE_TABLES;
use crate::memory::tools::*;
use crate::memory::VIRTUAL_OFFSET;
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

    /// trick to always map the pages tables from address 0xFFC...
    /// See [Osdev](https://wiki.osdev.org/Memory_Management_Unit)
    pub unsafe fn self_map_tricks(&mut self, cr3: PhysicalAddr) {
        self[1023] = Default::default();
        self[1023].set_entry_addr(cr3);
        self[1023] |= Entry::PRESENT | Entry::READ_WRITE;
    }

    pub fn set_page_tables(&mut self, offset: usize, page_tables: &[PageTable]) {
        for (i, pt) in page_tables.iter().enumerate() {
            self[offset + i] = Default::default();
            self[offset + i].set_entry_addr(PhysicalAddr(pt.as_ref().as_ptr() as usize - VIRTUAL_OFFSET));
            self[offset + i] |= Entry::PRESENT | Entry::READ_WRITE;
        }
    }

    /// use the self referencing trick. so must be called when paging is enabled and after self_map_tricks has been called
    #[inline(always)]
    pub unsafe fn map_page(
        &mut self,
        virtp: Page<VirtualAddr>,
        physp: Page<PhysicalAddr>,
        entry: Entry,
    ) -> Result<(), MemoryError> {
        let pd_index = virtp.pd_index();
        let page_table = &mut *((0xFFC00000 + pd_index * 4096) as *mut PageTable);
        page_table.map_page(virtp, physp, entry)
    }

    //TODO: check overflow
    pub unsafe fn map_range_page(
        &mut self,
        virtp: Page<VirtualAddr>,
        physp: Page<PhysicalAddr>,
        nb_pages: NbrPages,
        entry: Entry,
    ) -> Result<(), MemoryError> {
        for (virtp, physp) in (virtp..virtp + nb_pages).iter().zip((physp..physp + nb_pages).iter()) {
            self.map_page(virtp, physp, entry)?;
        }
        Ok(())
    }

    pub unsafe fn map_range_page_init(
        &mut self,
        virtp: Page<VirtualAddr>,
        physp: Page<PhysicalAddr>,
        nb_pages: NbrPages,
        entry: Entry,
    ) -> Result<(), MemoryError> {
        for (virtp, physp) in (virtp..virtp + nb_pages).iter().zip((physp..physp + nb_pages).iter()) {
            let pd_index = virtp.pd_index();
            let page_table = &mut PAGE_TABLES[pd_index];
            page_table.map_page(virtp, physp, entry)?;
        }
        Ok(())
    }

    pub unsafe fn unmap_page(&mut self, virtp: Page<VirtualAddr>) -> Result<(), MemoryError> {
        let pd_index = virtp.pd_index();
        let page_table = &mut *((0xFFC00000 + pd_index * 4096) as *mut PageTable);
        page_table.unmap_page(virtp)
    }

    //TODO: check overflow
    pub unsafe fn unmap_range_page(&mut self, virtp: Page<VirtualAddr>, nb_pages: NbrPages) -> Result<(), MemoryError> {
        for p in (virtp..virtp + nb_pages).iter() {
            self.unmap_page(p)?;
        }
        Ok(())
    }

    pub unsafe fn physical_page(&self, vaddr: Page<VirtualAddr>) -> Option<Page<PhysicalAddr>> {
        let pd_index = vaddr.pd_index();
        let pt_index = vaddr.pt_index();

        // TODO: Change that with the trick
        let page_table = &*((0xFFC00000 + pd_index * 4096) as *mut PageTable);

        if page_table[pt_index].contains(Entry::PRESENT) {
            Some(page_table[pt_index].entry_page())
        } else {
            None
        }
    }
    pub unsafe fn physical_addr(&self, vaddr: VirtualAddr) -> Option<PhysicalAddr> {
        self.physical_page(vaddr.into()).map(|v| v.into())
    }
    // pub unsafe fn load_current_page_directory(ptr: *mut PageDirectory) {
    //     Cr3::write(ptr as usize);
    // }

    // pub unsafe fn get_current_page_directory() -> *mut PageDirectory {
    //     Cr3::read() as *mut PageDirectory
    // }
    // /// This is a trick that ensures that the page tables are mapped into virtual memory.
    // /// The idea is that the last Entry points to self, viewed as a Page Table.
    // /// It means that the Virtual Addresses of the PageTables have their 10-higher bits set.
    // /// The range of bits [12..22] then describes the index inside the PageDirectory, that is the index of the PageTable itself.
    // /// Then the range of bits [0..12] describes the offset inside the PageTable, which is fine since a PageTable is exactly 4096 bytes.

    // pub fn get_page_from_vaddr(&self, vaddr: u32) -> Option<&PageTableEntry> {
    //     let pdindex = (vaddr >> 22) as usize;
    //     let ptindex = ((vaddr >> 12) & 0x0fff) as usize;

    //     if !self[pdindex].present() {
    //         return None;
    //     }

    //     let page_table = unsafe { &mut *(self[pdindex].entry_addr() as *mut PageTable) };

    //     Some(&page_table[ptindex])
    // }
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
