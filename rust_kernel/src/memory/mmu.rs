use crate::memory::tools::*;

pub mod page_directory;
pub use page_directory::PageDirectory;

pub mod page_table;
use page_table::PageTable;

mod entry;
pub use entry::Entry;

pub static mut PAGE_TABLES: [PageTable; 1024] = [PageTable::new(); 1024];

extern "C" {
    pub fn _enable_paging(addr: Phys);
    fn _enable_pse();
}
