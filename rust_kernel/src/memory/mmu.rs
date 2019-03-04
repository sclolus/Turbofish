pub mod page_directory;
mod page_directory_entry;
use page_directory::PageDirectory;
use page_directory_entry::PageDirectoryEntry;

pub mod page_table;
pub mod page_table_entry;
use page_table::PageTable;

pub use page_table_entry::PageTableEntry;

pub static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

pub static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

extern "C" {
    pub fn _enable_paging_with_cr(addr: *mut PageDirectoryEntry);
    pub fn _enable_paging();
    pub fn _disable_paging();
    fn _enable_pse();
}
