//! This module contains code related to the Page Tables in the MMU.
use super::invalidate_page;
use super::Entry;
use crate::memory::tools::*;
use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

/// This is the representation of the intermediate paging structure.
/// A PageTable is composed of 1024 Entry_ies.
/// This structure must be 4-KiB aligned.
#[repr(C, align(4096))]
#[derive(Copy, Clone)]
pub struct PageTable {
    entries: [Entry; 1024],
}

impl PageTable {
    /// This fonction creates a PageTable at addr `page_directory_addr`
    pub const fn new() -> Self {
        Self {
            entries: [Entry::new(); 1024],
        }
    }

    #[inline(always)]
    pub fn map_page(&mut self, virtp: Page<Virt>, physp: Page<Phys>, entry: Entry) -> Result<()> {
        let pt_index = virtp.pt_index();

        if self[pt_index].contains(Entry::PRESENT) {
            return Err(MemoryError::AlreadyMapped);
        }

        self[pt_index] = entry;
        self[pt_index].set_entry_page(physp);
        Ok(())
    }

    /// set the page to 0 and invalidate the tlb
    #[inline(always)]
    pub fn unmap_page(&mut self, virtp: Page<Virt>) -> Result<()> {
        let pt_index = virtp.pt_index();
        if !self[pt_index].contains(Entry::PRESENT) {
            return Err(MemoryError::AlreadyUnMapped);
        }

        invalidate_page(virtp);

        self[pt_index] = Default::default();
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
    T: SliceIndex<[Entry]>,
{
    type Output = T::Output;

    #[inline]
    fn index(&self, idx: T) -> &Self::Output {
        idx.index(&self.entries)
    }
}

/// The PageTable implements IndexMut which enables us to use the syntax: `pd[index] = SomeEntry`
/// instead of `pd.entries[index] = SomeEntry` in a mutable context.
/// This generic implementation also enables us to use the syntax pd[n..m] or any other Range slice indexing.
///
/// # Panics
/// Panics if `index` is outside of the PageTable, that is, if index >= PageTable.entries.len()
impl<'a, T> IndexMut<T> for PageTable
where
    T: SliceIndex<[Entry]>,
{
    #[inline]
    fn index_mut(&mut self, idx: T) -> &mut Self::Output {
        idx.index_mut(&mut self.entries)
    }
}

impl AsRef<[Entry]> for PageTable {
    fn as_ref(&self) -> &[Entry] {
        &self.entries
    }
}

impl AsMut<[Entry]> for PageTable {
    fn as_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }
}
