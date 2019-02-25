/// This module contains code related to the Page Tables in the MMU.
use bit_field::BitField;
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::SliceIndex;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(super) struct PageTableEntry {
    // Should this be u32 or usize ? I guess u32 is more accurate but...
    inner: usize,
}

impl PageTableEntry {
    #[allow(dead_code)]
    pub const fn new() -> Self {
        unsafe { Self { inner: 0 } }
    }

    pub fn set_physical_address(&mut self, addr: usize) -> &mut Self {
        assert!(addr.get_bits(0..12) == 0);

        self.inner.set_bits(12..32, addr.get_bits(12..32));
        self
    }

    pub fn physical_address(&self) -> usize {
        self.inner.get_bits(12..32) << 12
    }

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the present bit of the entry.
If set, indicates that the page is currently in memory.
If not set, then the CPU will page fault if the corresponding virtual address is dereferenced"],
        #[doc = "Gets the value of the present bit of the entry.
If set, indicates that the page is currently in memory.
If not set, then the CPU will page fault if the corresponding virtual address is dereferenced"],
        present,
        set_present,
        0,
        inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the read_write bit of the entry.
If the bit is set, the page is read/write.
Otherwise when it is not set, the page is read-only. The WP bit in CR0 determines if this is only applied to userland, always giving the kernel write access (the default) or both userland and the kernel."],
        #[doc = "Gets the read_write bit of the entry.
If the bit is set, the page is read/write.
Otherwise when it is not set, the page is read-only. The WP bit in CR0 determines if this is only applied to userland, always giving the kernel write access (the default) or both userland and the kernel."],
        read_write, set_read_write, 1, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the user bit of the entry.
When set, this bit indicate that the page can be accessed by anyone.
When not set, only the supervisor can access this page."],
        #[doc = "Gets the value of the user bit of the entry
When set, this bit indicate that the page can be accessed by anyone.
When not set, only the supervisor can access this page."],
        user_bit, set_user_bit, 2, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the write-through bit of the entry.
It controls the `write-through` ability of the page when set.
When not set, `write-back` is enabled instead."],
        #[doc = "Gets the value of the write-through bit of the entry.
It controls the `write-through` ability of the page when set.
When not set, `write-back` is enabled instead."],
        write_through, set_write_through, 3, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the cache disabled bit of the entry.
If set, then the page will not be cached by the CPU.
If not set, then the page will be cached if possible."],
        #[doc = "Getse the value of the cache disabled bit of the entry.
If set, then the page will not be cached by the CPU.
If not set, then the page will be cached if possible."],
        cache_disabled, set_cache_disabled, 4, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the accessed bit of the entry.
If this is set, then the page was accessed by the cpu.
If this is not set, then the page was not accessed.
This flag is set by the cpu when a page in the directory is accessed.
It won't be cleared by the CPU, so it is the responsability of the kernel to clear it, if the kernel needs it at all"],
        #[doc = "Gets the value of the accessed bit of the entry.
If this is set, then the page was accessed by the cpu.
If this is not set, then the page was not accessed.
This flag is set by the cpu when a page in the directory is accessed.
It won't be cleared by the CPU, so it is the responsability of the kernel to clear it, if the kernel needs it at all"],
        accessed, set_accessed, 5, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the Dirty flag of the entry.
If set, the page has been writen to.
If not set, it was not.
This flag is not updated by the CPU, and once set will not unset itself."],
        #[doc = "Gets the value of the Dirty flag of the entry.
If set, the page has been writen to.
If not set, it was not.
This flag is not updated by the CPU, and once set will not unset itself."],
        dirty, set_dirty, 6, inner);

    ///0, if PAT is supported, shall indicate the memory type. Otherwise, it must be 0.
    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the zero flag of the entry.
If PAT is supported, it shall indicate the memory type.
If not, it must be zero."],
        #[doc = "Gets the value of the zero flag of the entry.
If PAT is supported, it shall indicate the memory type.
If not, it must be zero."],
        zero, set_zero, 0, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the Global flag of the entry.
 if set, prevents the TLB from updating the address in its cache if CR3 is reset. Note, that the page global enable bit in CR4 must be set to enable this feature."],
        #[doc = "Gets the value of the Global flag of the entry.
 if set, prevents the TLB from updating the address in its cache if CR3 is reset. Note, that the page global enable bit in CR4 must be set to enable this feature."],
        global, set_global, 8, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the available_1 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        #[doc = "Gets the value of the available_1 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        available_1, set_available_1, 9, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the available_2 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        #[doc = "Gets the value of the available_2 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        available_2, set_available_2, 10, inner);

    gen_builder_pattern_bitfields_methods!(
        #[doc = "Sets the value of the available_3 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        #[doc = "Gets the value of the available_3 flag of the entry.
This is flag is currently unused and can be assigned to any role it may fulfill"],
        available_3, set_available_3, 11, inner);
}

/// This is the representation of the intermediate paging structure.
/// A PageTable is composed of 1024 PageTableEntry_ies.
/// This structure must be 4-KiB aligned.
#[repr(C, align(4096))]
#[derive(Copy, Clone)]
pub(super) struct PageTable {
    entries: [PageTableEntry; 1024],
}

impl PageTable {
    pub const DEFAULT_PAGE_TABLE_SIZE: usize = 1024;

    /// This fonction creates a PageTable at addr `page_directory_addr`
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self { entries: [PageTableEntry::new(); 1024] }
    }

    pub fn map_addr(&mut self, virt_addr: usize, phys_addr: usize) -> Result<(), ()> {
        assert!(virt_addr % 4096 == 0);
        assert!(phys_addr % 4096 == 0);

        let page_table_index = virt_addr.get_bits(12..22);

        if self[page_table_index].present() {
            return Err(());
        }

        //TODO: take custom flags
        self[page_table_index].set_read_write(true).set_present(true).set_physical_address(phys_addr);
        Ok(())
    }
}

/// The PageTable implements Index which enables us to use the syntax: `pd[index]`,
/// instead of `pd.entries[index]` in an immutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageTable, that is, if index >= PageTable.entries.len()
impl<'a, T> Index<T> for PageTable
where
    T: SliceIndex<[PageTableEntry]>,
{
    type Output = T::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        idx.index(&self.entries)
    }
}

/// The PageTable implements IndexMut which enables us to use the syntax: `pd[index] = SomePageTableEntry`
/// instead of `pd.entries[index] = SomePageTableEntry` in a mutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageTable, that is, if index >= PageTable.entries.len()
impl<'a, T> IndexMut<T> for PageTable
where
    T: SliceIndex<[PageTableEntry]>,
{
    #[inline]
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        idx.index_mut(&mut self.entries)
    }
}

impl AsRef<[PageTableEntry]> for PageTable {
    fn as_ref(&self) -> &[PageTableEntry] {
        &self.entries
    }
}

impl AsMut<[PageTableEntry]> for PageTable {
    fn as_mut(&mut self) -> &mut [PageTableEntry] {
        &mut self.entries
    }
}
