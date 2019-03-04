use super::page_table::PageTable;
use bit_field::BitField;

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

    pub unsafe fn get_page_table(&self) -> Option<*mut PageTable> {
        if self.present() {
            Some(self.entry_addr() as *mut PageTable)
        } else {
            None
        }
    }
}
