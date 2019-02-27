use super::PAGE_SIZE;
use super::{PhysicalAddr, VirtualAddr};
/// This module contains the code related to the page directory and its page directory entries, which are the highest abstraction paging-related data structures (for the cpu)
/// See https://wiki.osdev.org/Paging for relevant documentation.
use bit_field::BitField;
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

#[repr(C)] // this should be equivalent to `transparent` I hope
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PageDirectoryEntry {
    // Should this be u32 or usize ? I guess u32 is more accurate but...
    inner: usize,
}

impl PageDirectoryEntry {
    pub const fn new() -> Self {
        unsafe { Self { inner: 0 } }
    }

    /// Sets the present bit of the entry.
    /// If set, indicates that the page directory is currently in memory.
    /// If not set, then the CPU will ignore this directory in its search for an address translation.
    #[allow(dead_code)]
    pub fn set_present(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(0, bit);
        self
    }

    /// Gets the value of the present bit of the entry.
    /// If set, indicates that the page directory is currently in memory.
    /// If not set, then the CPU will ignore this directory in its search for an address translation.
    #[allow(dead_code)]
    pub fn present(&self) -> bool {
        self.inner.get_bit(0)
    }

    #[allow(dead_code)]
    pub fn set_read_write(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(1, bit);
        self
    }

    #[allow(dead_code)]
    pub fn read_write(&self) -> bool {
        self.inner.get_bit(1)
    }

    /// Sets the user bit of the entry. /// When set, this bit indicate that the page directory contains pages that can be accessed by everyone.
    /// When not set, only the supervisor can access those pages.
    #[allow(dead_code)]
    pub fn set_user_bit(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(2, bit);
        self
    }

    /// Gets the value of the user bit of the entry.
    /// When set, this bit indicate that the page directory contains pages that can be accessed by everyone.
    /// When not set, only the supervisor can access those pages.
    #[allow(dead_code)]
    pub fn user_bit(&self) -> bool {
        self.inner.get_bit(2)
    }

    /// Controls the `write-through` ability of the page when set.
    /// When not set, `write-back` is enabled instead.
    #[allow(dead_code)]
    pub fn set_write_through(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(3, bit);
        self
    }

    #[allow(dead_code)]
    pub fn write_through(&self) -> bool {
        self.inner.get_bit(3)
    }

    /// Sets the cache disabled flag.
    /// If this is set, then the page will not be cached by the CPU.
    /// If this is not set, then the page will be cached if possible.
    #[allow(dead_code)]
    pub fn set_cache_disable(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(4, bit);
        self
    }

    /// Gets the value of the cache disable bit of the entry.
    /// If this is set, then the page will not be cached by the CPU.
    /// If this is not set, then the page will be cached if possible.
    #[allow(dead_code)]
    pub fn cache_disable(&self) -> bool {
        self.inner.get_bit(4)
    }

    /// Sets the value of the accessed bit of the entry flag.
    /// If this is set, then a page of the directory was accessed by the cpu.
    /// if not set, no page was accessed.
    /// This flag is set by the cpu when a page in the directory is accessed.
    /// It won't be cleared by the CPU, so it is the responsability of the kernel to clear it, If the kernel needs it at all.
    #[allow(dead_code)]
    pub fn set_accessed(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(5, bit);
        self
    }

    #[allow(dead_code)]
    pub fn accessed(&self) -> bool {
        self.inner.get_bit(5)
    }

    /// Sets the page_size bit in the entry to value of `bit`
    /// Setting the S bit makes the page directory entry point directly to a 4-MiB page.
    /// There is no paging table involved in the address translation.
    /// Note: With 4-MiB pages, bits 21 through 12 are reserved! Thus, the physical address must also be 4-MiB-aligned.
    #[allow(dead_code)]
    pub fn set_page_size(&mut self, bit: bool) -> &mut Self {
        self.inner.set_bit(7, bit);
        self
    }

    /// Gets the page_size bit in the entry to value of `bit`.
    /// Setting the S bit makes the page directory entry point directly to a 4-MiB page.
    /// There is no paging table involved in the address translation.
    /// Note: With 4-MiB pages, bits 21 through 12 are reserved! Thus, the physical address must also be 4-MiB-aligned.
    #[allow(dead_code)]
    pub fn page_size(&self) -> bool {
        self.inner.get_bit(7)
    }

    /// Sets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[allow(dead_code)]
    pub fn set_entry_addr(&mut self, addr: usize) -> &mut Self {
        // asserts that if the page_size bit is set for this entry, the set addr is 4-MiB aligned.
        assert!(if self.page_size() { addr.get_bits(0..22) == 0 } else { addr.get_bits(0..12) == 0 });

        self.inner.set_bits(12..32, addr.get_bits(12..32));
        self
    }

    /// Gets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[allow(dead_code)]
    pub fn entry_addr(&self) -> usize {
        (self.inner.get_bits(12..32) as usize) << 12
    }

    /// This sets the 3 available bits of the entry.
    /// Currently this is more a placeholder then a definitive implementation. It should be decided what is done with those bits.
    #[allow(dead_code)]
    pub fn set_available_field(&mut self, bits: u8) -> &mut Self {
        self.inner.set_bits(9..12, bits as usize);
        self
    }

    #[allow(dead_code)]
    pub fn available_field(&self) -> u8 {
        self.inner.get_bits(9..12) as u8
    }
}

/// This is the representation of the topmost paging structure.
/// It is composed of 1024 PageDirectoryEntry.
/// This structure must be 4-KiB aligned.
#[repr(C, align(4096))]
pub(super) struct PageDirectory {
    entries: [PageDirectoryEntry; 1024],
}

/// The PageDirectory implements Index which enables us to use the syntax: `pd[index]`,
/// instead of `pd.entries[index]` in an immutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageDirectory, that is, if index >= PageDirectory.entries.len()
impl<'a, T> Index<T> for PageDirectory
where
    T: SliceIndex<[PageDirectoryEntry]>,
{
    type Output = T::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        idx.index(&self.entries)
    }
}

/// The PageDirectory implements IndexMut which enables us to use the syntax: `pd[index] = SomePageDirectoryEntry`
/// instead of `pd.entries[index] = SomePageDirectoryEntry` in a mutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageDirectory, that is, if index >= PageDirectory.entries.len()
impl<'a, T> IndexMut<T> for PageDirectory
where
    T: SliceIndex<[PageDirectoryEntry]>,
{
    #[inline]
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        idx.index_mut(&mut self.entries)
    }
}

impl AsRef<[PageDirectoryEntry]> for PageDirectory {
    fn as_ref(&self) -> &[PageDirectoryEntry] {
        &self.entries
    }
}

impl AsMut<[PageDirectoryEntry]> for PageDirectory {
    fn as_mut(&mut self) -> &mut [PageDirectoryEntry] {
        &mut self.entries
    }
}

use super::page_table::*;
impl PageDirectory {
    pub const DEFAULT_PAGE_DIRECTORY_SIZE: usize = 1024;

    /// This fonction creates a PageDirectory at addr `page_directory_addr` of size (in elements) of `size`.
    pub const fn new() -> Self {
        Self { entries: [PageDirectoryEntry::new(); 1024] }
    }

    /// This is a trick that ensures that the page tables are mapped into virtual memory.
    /// The idea is that the last PageDirectoryEntry points to self, viewed as a Page Table.
    /// It means that the Virtual Addresses of the PageTables have their 10-higher bits set.
    /// The range of bits [12..22] then describes the index inside the PageDirectory, that is the index of the PageTable itself.
    /// Then the range of bits [0..12] describes the offset inside the PageTable, which is fine since a PageTable is exactly 4096 bytes.
    #[allow(dead_code)]
    pub fn self_map_tables(&mut self) {
        let entry =
            *PageDirectoryEntry::new().set_present(true).set_read_write(true).set_entry_addr(self as *const _ as usize);

        self[1023] = entry;
    }

    #[allow(dead_code)]
    pub fn get_page_from_vaddr(&self, vaddr: u32) -> Option<&PageTableEntry> {
        let pdindex = (vaddr >> 22) as usize;
        let ptindex = ((vaddr >> 12) & 0x0fff) as usize;

        if !self[pdindex].present() {
            return None;
        }

        let page_table = unsafe { &mut *(self[pdindex].entry_addr() as *mut PageTable) };

        Some(&page_table[ptindex])
    }

    pub unsafe fn remap_addr(&mut self, virt_addr: usize, phys_addr: usize) -> Result<(), ()> {
        assert_eq!(virt_addr % PAGE_SIZE, 0);
        assert_eq!(phys_addr % PAGE_SIZE, 0);

        let page_dir_index = virt_addr.get_bits(22..32);

        // Do not uncomment.
        // self[page_dir_index] = *PageDirectoryEntry::new().set_present(true).set_read_write(true);

        if page_dir_index == 1023 {
            println!("{:x}", virt_addr);
            println!("{:x}", phys_addr);
        }
        let page_table = &mut *(self[page_dir_index].entry_addr() as *mut PageTable);

        assert_eq!(
            page_table as *const PageTable as usize,
            self[page_dir_index].entry_addr() as *const PageTable as usize
        );

        use super::PAGE_TABLES;
        if PAGE_TABLES.iter().any(|iter_table| {
            // println!("{:p} != {:p}", iter_table, page_table);
            iter_table as *const _ == page_table as *const _
        }) == false
        {
            for (_, _table) in PAGE_TABLES.iter().enumerate() {
                // println!(
                //     "{} -> {:p} != {:p} is not found in PAGE_TABLES",
                //     index, table as *const _, page_table as *const _
                // );
            }
        }
        assert!(PAGE_TABLES.iter().any(|iter_table| {
            // println!("{:p} != {:p}", iter_table, page_table);
            iter_table as *const _ == page_table as *const _
        }));

        page_table.map_addr(virt_addr, phys_addr)?;
        Ok(())
    }

    //TODO: check overflow
    pub unsafe fn remap_range_addr(
        &mut self,
        virt_addr: VirtualAddr,
        phys_addr: PhysicalAddr,
        nb_pages: usize,
    ) -> Result<(), ()> {
        assert_eq!(virt_addr.0 % PAGE_SIZE, 0);
        assert_eq!(phys_addr.0 % PAGE_SIZE, 0);
        for offset in (0..nb_pages).map(|offset| offset * PAGE_SIZE) {
            self.remap_addr(virt_addr.0 + offset, phys_addr.0 + offset)?;
        }
        Ok(())
    }

    pub fn set_page_tables(&mut self, offset: usize, page_tables: &[PageTable]) {
        for (i, pt) in page_tables.iter().enumerate() {
            self[offset + i].set_entry_addr(pt.as_ref().as_ptr() as usize);
            self[offset + i].set_present(true);
        }
    }

    pub unsafe fn load_current_page_directory(ptr: *mut PageDirectory) {
        use crate::registers::Cr3;

        Cr3::write(ptr as usize);
    }

    pub unsafe fn get_current_page_directory() -> *mut PageDirectory {
        use crate::registers::Cr3;

        Cr3::read() as *mut PageDirectory
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new();
    static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] =
        [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

    #[test]
    fn test_entry_addrs() {
        unsafe {
            PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);
            for (pd, pt) in PAGE_DIRECTORY.as_ref().iter().zip(PAGE_TABLES.iter()) {
                assert_eq!(pd.entry_addr(), pt as *const _ as usize);
            }
        }
    }
}
