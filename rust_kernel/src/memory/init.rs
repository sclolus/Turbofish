use crate::memory::kernel_allocator::init_physical_allocator;
use crate::memory::mmu::_enable_paging_with_cr;
use crate::memory::mmu::*;
use crate::memory::tools::*;
use crate::memory::*;

pub unsafe fn init_memory_system() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());
    PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);
    println!("step 1");
    PAGE_DIRECTORY.map_range_addr(VirtualAddr(0), PhysicalAddr(0), NbrPages::_64MB).unwrap();
    println!("step 2");
    PAGE_DIRECTORY.map_range_addr(VirtualAddr(0xc0000000), PhysicalAddr(0xc0000000), NbrPages::_1GB).unwrap();
    PAGE_DIRECTORY.map_range_addr(VirtualAddr(0x90000000), PhysicalAddr(0x90000000), NbrPages::_8MB).unwrap();

    // for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
    //     dir_entry.set_present(true);
    //     debug_assert!(dir_entry.present() == true);
    // }

    //println!("before enable paging");
    //println!("{:X?}", PAGE_DIRECTORY.as_mut().as_mut_ptr());
    //println!("{:X?}", PAGE_DIRECTORY[0]);
    //println!("{:X?}", *(PAGE_DIRECTORY[0].entry_addr() as *const u8 as *const PageTableEntry));
    //_enable_paging_with_cr(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    _enable_paging_with_cr(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    println!("after setting cr3");
    init_physical_allocator();
    /*
    let toto: *mut u8;
    toto = 0x60000000 as *mut u8;
    unsafe {
        *toto = 42;
    }
    */
    println!("toto");

    Ok(())
}
