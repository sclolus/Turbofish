use crate::memory::tools::*;
use bit_field::BitField;
use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    #[repr(C)]
    pub struct Entry: u32 {
        /// If set, indicates that the page directory is currently in memory.
        /// If not set, then the CPU will ignore this directory in its search for an address translation.
        const PRESENT = 1 << 0;

        const READ_WRITE = 1 << 1;

        /// When set, this bit indicate that the page directory contains pages that can be accessed by everyone.
        /// When not set, only the supervisor can access those pages.
        const USER = 1 << 2;

        /// Controls the `write-through` ability of the page when set.
        /// When not set, `write-back` is enabled instead.
        const WRITE_THROUGH = 1 << 3;

        /// If this is set, then the page will not be cached by the CPU.
        /// If this is not set, then the page will be cached if possible.
        const CACHE_DISABLE = 1 << 4;

        /// If this is set, then a page of the directory was accessed by the cpu.
        /// if not set, no page was accessed.
        /// This flag is set by the cpu when a page in the directory is accessed.
        /// It won't be cleared by the CPU, so it is the responsability of the kernel to clear it, If the kernel needs it at all.
        const ACCESSED = 1 << 5;

        /// WARN: Page Table Specific
        /// If set, the page has been writen to.
        /// If not set, it was not.
        /// This flag is not updated by the CPU, and once set will not unset itself.
        const DIRTY = 1 << 6;

        /// WARN: Page Table Specific
        /// If PAT is supported, it shall indicate the memory type.
        /// If not, it must be zero.
        const ZERO = 1 << 7;

        /// WARN: Page Directory specific, ignored in Page Table
        /// Setting the S bit makes the page directory entry point directly to a 4-MiB page.
        /// There is no paging table involved in the address translation.
        /// Note: With 4-MiB pages, bits 21 through 12 are reserved! Thus, the physical address must also be 4-MiB-aligned.
        const PAGE_SIZE = 1 << 7;

        /// WARN: Page Table specific, ignored in Page Directory
        /// if set, prevents the TLB from updating the address in its cache if CR3 is reset. Note, that the page global enable bit in CR4 must be set to enable this feature.
        const GLOBAL = 1 << 8;
    }
}

impl Entry {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    /// Sets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[allow(dead_code)]
    pub fn set_entry_addr(&mut self, addr: Phys) -> &mut Self {
        // asserts that if the page_size bit is set for this entry, the set addr is 4-MiB aligned.
        assert!(if self.contains(Entry::PAGE_SIZE) {
            addr.0.get_bits(0..22) == 0
        } else {
            addr.0.get_bits(0..12) == 0
        });
        self.bits.set_bits(12..32, addr.0.get_bits(12..32) as u32);
        self
    }

    /// Sets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[inline(always)]
    pub fn set_entry_page(&mut self, page: Page<Phys>) -> &mut Self {
        self.bits.set_bits(12..32, page.number as u32);
        self
    }

    #[inline(always)]
    pub fn entry_page(&self) -> Page<Phys> {
        Page::new(self.bits.get_bits(12..32) as usize)
    }

    /// Gets the address field of the entry.
    /// When the page_size bit is not set, the address is a 4-kb aligned address pointing to a Page Table.
    /// When the page_size bit is set, the address instead directly points to a 4-MiB page, so no Page Table is then involved.
    #[allow(dead_code)]
    pub fn entry_addr(&self) -> Phys {
        Phys((self.bits.get_bits(12..32) as usize) << 12)
    }

    /// This sets the 3 available bits() of the entry.
    /// Currently this is more a placeholder then a definitive implementation. It should be decided what is done with those bits().
    #[allow(dead_code)]
    pub fn set_available_field(&mut self, bits: u8) -> &mut Self {
        self.bits.set_bits(9..12, bits as u32);
        self
    }

    #[allow(dead_code)]
    pub fn available_field(&self) -> u8 {
        self.bits().get_bits(9..12) as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_entry() {
        let mut entry = Entry::PRESENT | Entry::READ_WRITE;
        assert!(entry.contains(Entry::PRESENT));
        entry.set_entry_addr(Phys(0x1000));
        assert!(entry.contains(Entry::PRESENT));
    }
}
