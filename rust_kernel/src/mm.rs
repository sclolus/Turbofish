/// This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
/// See https://wiki.osdev.org/Paging for relevant documentation.
pub mod page_directory;
pub mod page_table;

// pub use page_directory::PageDirectoryEntry;
// pub use page_table::PageTableEntry;

use page_directory::{PageDirectory, PageDirectoryEntry};
use page_table::{PageTable, PageTableEntry};

#[allow(dead_code)]
static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

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
