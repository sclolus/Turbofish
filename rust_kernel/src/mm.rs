/// This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
/// See https://wiki.osdev.org/Paging for relevant documentation.
pub mod page_directory;
pub mod page_table;
use bit_field::BitField;
use core::ops::Range;

// pub use page_directory::PageDirectoryEntry;
// pub use page_table::PageTableEntry;

use page_directory::{PageDirectory, PageDirectoryEntry};
use page_table::{PageTable, PageTableEntry};

#[allow(dead_code)]
static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

/// set the 16 first Mib of virtual address = real address
pub fn auto_ref_kernel_base() {
    let dir = &mut PAGE_DIRECTORY;
    let table = &mut PAGE_TABLES;
    let mut offset = 0;
    for j in 0..16 {
        offset = j * 1024;

        dir.entries[j] = *PageDirectoryEntry::new()
            .set_present(true)
            .set_read_write(true)
            .set_entry_addr(&table[j] as *const PageTable as usize);

        for (i, e) in table[j].as_mut().iter_mut().enumerate() {
            *e = *PageTableEntry::new()
                .set_present(true)
                .set_read_write(true)
                .set_physical_address(((offset + i) as u32) << 12);
        }
    }
}

pub fn remap_addr(virt_addr: u32, phys_addr: u32) {
    assert_eq!(virt_addr % 4096, 0);
    let dir = &mut PAGE_DIRECTORY;
    let table = &mut PAGE_TABLES;
    let page_dir_index = virt_addr.get_bits(22..32);
    dir.entries[page_dir_index as usize] = *PageDirectoryEntry::new()
        .set_present(true)
        .set_read_write(true)
        .set_entry_addr(&table[page_dir_index as usize] as *const PageTable as u32);

    let page_table_index = virt_addr.get_bits(12..22);
    table[page_dir_index as usize][page_table_index as usize] =
        *PageTableEntry::new().set_present(true).set_read_write(true).set_physical_address(phys_addr << 12);
}

pub fn remap_range_addr(virt_addr_range: Range<u32>, phys_addr_range: Range<u32>) {
    assert_eq!(virt_addr_range.start % 4096, 0);
    assert_eq!(phys_addr_range.start % 4096, 0);
    for (virt, phys) in virt_addr_range.zip(phys_addr_range).step_by(4096) {
        remap_addr(virt, phys);
    }
}

pub unsafe fn init_paging() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());

    for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
        dir_entry.set_present(false);
        assert!(dir_entry.present() == false);
    }

    let first_directory_entry = PageDirectoryEntry::new()
        .set_present(true)
        .set_read_write(true)
        .set_entry_addr((PAGE_TABLES[0].as_ref().as_ptr() as usize) >> 12);

    println!("ptr for first entry: {:x}", first_directory_entry.entry_addr() << 12);
    PAGE_DIRECTORY.set_directory_entry(0, first_directory_entry);

    let init_page_table = &mut PAGE_TABLES[0];
    for index in 0u32..1024u32 {
        let page_entry =
            *PageTableEntry::new().set_global(true).set_present(true).set_read_write(true).set_physical_address(index);

        init_page_table.set_page_entry(index as usize, page_entry);
    }

    println!("arg to enable_paging: {:p}", PAGE_DIRECTORY.as_mut().as_mut_ptr());
    // _enable_paging(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    // loop {}

    Ok(())
}

extern "C" {
    fn _enable_paging(addr: *mut PageDirectoryEntry);
    fn _enable_pse();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitfield() {
        let mut entry: PageDirectoryEntry = PageDirectoryEntry::new();

        assert_eq!(entry.present(), false);
        entry.set_present(true);
        assert_ne!(entry.set_present(true), PageDirectoryEntry::new());
        assert_eq!(entry.present(), true);
    }
}
