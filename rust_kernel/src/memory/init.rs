use crate::memory::kernel_allocator::init_physical_allocator;
use crate::memory::mmu::_enable_paging_with_cr;
use crate::memory::mmu::*;
use crate::memory::tools::*;
use crate::memory::*;

pub unsafe fn init_memory_system() -> Result<(), ()> {
    PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);
    PAGE_DIRECTORY.map_range_page(VirtualAddr(0).into(), PhysicalAddr(0).into(), NbrPages::_64MB).unwrap();
    PAGE_DIRECTORY
        .map_range_page(VirtualAddr(0xc0000000).into(), PhysicalAddr(0xc0000000).into(), NbrPages::_1GB)
        .unwrap();
    PAGE_DIRECTORY
        .map_range_page(VirtualAddr(0x90000000).into(), PhysicalAddr(0x90000000).into(), NbrPages::_8MB)
        .unwrap();

    _enable_paging_with_cr(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    init_physical_allocator();

    Ok(())
}
