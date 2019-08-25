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
    pub fn _read_cr3() -> Phys;
    fn _enable_pse();
    fn _invlpg(addr: Virt);
    fn _invlpg_range(addr: Virt, nbr_pages: NbrPages);
}

#[inline(always)]
pub fn invalidate_page(page: Page<Virt>) {
    unsafe {
        _invlpg(page.into());
    }
}

#[inline(always)]
pub fn invalidate_page_range(page: Page<Virt>, nbr_pages: NbrPages) {
    unsafe {
        _invlpg_range(page.into(), nbr_pages);
    }
}
