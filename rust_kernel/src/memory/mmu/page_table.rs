//! This module contains code related to the Page Tables in the MMU.
use super::page_table_entry::PageTableEntry;
use crate::memory::tools::*;
use bit_field::BitField;
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

/// This is the representation of the intermediate paging structure.
/// A PageTable is composed of 1024 PageTableEntry_ies.
/// This structure must be 4-KiB aligned.
#[repr(C, align(4096))]
#[derive(Copy, Clone)]
pub struct PageTable {
    entries: [PageTableEntry; 1024],
}

impl PageTable {
    /// This fonction creates a PageTable at addr `page_directory_addr`
    pub const fn new() -> Self {
        Self { entries: [PageTableEntry::new(); 1024] }
    }

    #[inline(always)]
    pub fn map_addr(&mut self, virt_addr: usize, phys_addr: usize) -> Result<(), MemoryError> {
        let page_table_index = virt_addr.get_bits(12..22);

        if self[page_table_index].present() {
            return Err(MemoryError::AlreadyMapped);
        }

        //TODO: take custom flags
        self[page_table_index] =
            *PageTableEntry::new().set_read_write(true).set_present(true).set_physical_address(phys_addr);
        Ok(())
    }

    #[inline(always)]
    pub fn unmap_addr(&mut self, virt_addr: usize) -> Result<(), MemoryError> {
        let page_table_index = virt_addr.get_bits(12..22);
        if !self[page_table_index].present() {
            return Err(MemoryError::AlreadyUnMapped);
        }

        //TODO: take custom flags
        self[page_table_index].set_present(false);
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
