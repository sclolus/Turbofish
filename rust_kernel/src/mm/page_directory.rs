/// This module contains the code related to the page directory and its page directory entries, which are the highest abstraction paging-related data structures (for the cpu)
/// See https://wiki.osdev.org/Paging for relevant documentation.
use bit_field::BitField;

#[repr(C)] // this should be equivalent to `transparent` I hope
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct PageDirectoryEntry {
    // Should this be u32 or usize ? I guess u32 is more accurate but...
    inner: u32,
}

impl PageDirectoryEntry {
    pub const fn new() -> Self {
        unsafe { Self { inner: 0 } }
    }

    /// Sets the present bit of the entry.
    /// If set, indicates that the page directory is currently in memory.
    /// If not set, then the CPU will ignore this directory in its search for an address translation.
    #[allow(dead_code)]
    pub fn set_present(mut self, bit: bool) -> Self {
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
    pub fn set_read_write(mut self, bit: bool) -> Self {
        self.inner.set_bit(1, bit);
        self
    }

    #[allow(dead_code)]
    pub fn read_write(&self) -> bool {
        self.inner.get_bit(1)
    }

    /// Sets the user bit of the entry.
    /// When set, this bit indicate that the page directory contains pages that can be accessed by everyone.
    /// When not set, only the supervisor can access those pages.
    #[allow(dead_code)]
    pub fn set_user_bit(mut self, bit: bool) -> Self {
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
    pub fn set_write_through(mut self, bit: bool) -> Self {
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
    pub fn set_cache_disable(mut self, bit: bool) -> Self {
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
    pub fn set_accessed(mut self, bit: bool) -> Self {
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
    pub fn set_page_size(mut self, bit: bool) -> Self {
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
    pub fn set_entry_addr(mut self, addr: usize) -> Self {
        // asserts that if the page_size bit is set for this entry, the set addr is 4-MiB aligned.
        assert!(if self.page_size() { addr.get_bits(0..12) == 0 } else { true });

        self.inner.set_bits(12..32, addr as u32);
        self
    }

    /// Gets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[allow(dead_code)]
    pub fn entry_addr(&self) -> usize {
        self.inner.get_bits(12..32) as usize
    }

    /// This sets the 3 available bits of the entry.
    /// Currently this is more a placeholder then a definitive implementation. It should be decided what is done with those bits.
    #[allow(dead_code)]
    pub fn set_available_field(mut self, bits: u8) -> Self {
        self.inner.set_bits(9..12, bits as u32);
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

use super::page_table::*;
use PageDirectoryError::*;
impl PageDirectory {
    pub const DEFAULT_PAGE_DIRECTORY_SIZE: usize = 1024;

    /// This fonction creates a PageDirectory at addr `page_directory_addr` of size (in elements) of `size`.
    pub const fn new() -> Self {
        Self { entries: [PageDirectoryEntry::new(); 1024] }
    }

    #[allow(dead_code)]
    pub fn set_directory_entry(&mut self, index: usize, entry: PageDirectoryEntry) -> Result<(), PageDirectoryError> {
        self.entries.get_mut(index).map_or(Err(ErrIndexOutOfBound), |slot| {
            *slot = entry;
            Ok(())
        })
    }

    #[allow(dead_code)]
    pub fn get_directory_entry(&self, index: usize) -> Result<PageDirectoryEntry, PageDirectoryError> {
        self.entries.get(index).map_or(Err(ErrIndexOutOfBound), |slot| Ok(*slot))
    }

    #[allow(dead_code)]
    pub fn get_page_from_vaddr(&self, vaddr: u32) // -> &PageTableEntry
    {
        let _pdindex = (vaddr >> 22) as usize;
        let _ptindex = ((vaddr >> 12) & 0x0fff) as usize;

        // &self.entries[_pdindex][_ptindex]
    }
}

// impl Deref<PageDirectoryEntry>

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

pub enum PageDirectoryError {
    ErrIndexOutOfBound,
    UnknownError,
}
