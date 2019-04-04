use crate::memory::tools::*;

pub mod page_directory;
pub use page_directory::PageDirectory;

pub mod page_table;
use page_table::PageTable;

mod entry;
pub use entry::Entry;

pub static mut BIOS_PAGE_TABLE: [PageTable; 1] = [PageTable::new(); 1];
pub static mut PAGE_TABLES: [PageTable; 255] = [PageTable::new(); 255];

extern "C" {
    pub fn _enable_paging(addr: Phys);
    fn _enable_pse();
    fn _invlpg(addr: Virt);
}

#[inline(always)]
pub fn invalidate_page(page: Page<Virt>) {
    unsafe {
        _invlpg(page.into());
    }
}
