use crate::memory::kernel_allocator::init_physical_allocator;
use crate::memory::mmu::page_directory::init_mmu;
//use crate::memory::mmu::_enable_paging_with_cr;
//use crate::memory::mmu::*;
//use crate::memory::tools::*;
//use crate::memory::*;

pub unsafe fn init_memory_system() -> Result<(), ()> {
    init_mmu();
    init_physical_allocator();

    Ok(())
}
