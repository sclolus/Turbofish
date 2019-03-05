pub mod page_directory;
use page_directory::PageDirectory;

pub mod page_table;
use page_table::PageTable;

mod page_entry;
use page_entry::Entry;

pub static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

pub static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

extern "C" {
    pub fn _enable_paging_with_cr(addr: *mut Entry);
    pub fn _enable_paging();
    pub fn _disable_paging();
    fn _enable_pse();
}
