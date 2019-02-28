//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
pub mod address;
pub use address::*;
pub mod buddy_allocator;
//pub mod page_alloc;
//use page_alloc::{AllocFlags, PageAllocator, PhysicalAllocatorType, VirtualAllocatorType, ALLOC_NORMAL};
pub mod nbr_pages;
pub mod page_directory;
pub mod page_table;
pub mod physical_allocator;
pub use nbr_pages::*;
use physical_allocator::{init_physical_allocator, PHYSICAL_ALLOCATOR};

//use bit_field::BitField;
#[allow(unused_imports)]
use buddy_allocator::*;
use core::convert::AsRef;

//Remove this eventually.
pub mod dummy_allocator;

pub const PAGE_SIZE: usize = 4096;
// const_assert!(PAGE_SIZE.is_power_of_two());

use page_directory::{PageDirectory, PageDirectoryEntry};
use page_table::PageTable;

extern "C" {
    pub fn _enable_paging_with_cr(addr: *mut PageDirectoryEntry);
    pub fn _enable_paging();
    pub fn _disable_paging();
    fn _enable_pse();
}

extern "C" {
    static __start_text: u8;
    static __end_text: u8;

    static __start_boot: u8;
    static __end_boot: u8;

    static __start_rodata: u8;
    static __end_rodata: u8;

    static __start_data: u8;
    static __end_data: u8;

    static __start_debug: u8;
    static __end_debug: u8;

    static __start_bss: u8;
    static __end_bss: u8;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// This might also significate that the allocator has no memory for internal storage of metadatas left.
    OutOfMem,
    OutOfBound,
    AlreadyOccupied,
    NotSatifiableFlags,
    AlreadyMapped,
    CannotFree,
}

#[allow(dead_code)]
static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];
static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

use core::alloc::{GlobalAlloc, Layout};

pub struct MemoryManager;

unsafe impl GlobalAlloc for MemoryManager {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let page_allocator = PHYSICAL_ALLOCATOR.as_mut().unwrap();

        page_allocator.alloc_kernel(layout.size()).unwrap().0 as *mut u8 //.unwrap_or(PhysicalAddr(0x0)).0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let page_allocator = PHYSICAL_ALLOCATOR.as_mut().unwrap();
        page_allocator.free_kernel(PhysicalAddr(ptr as usize), layout.size()).unwrap(); //.unwrap_or(PhysicalAddr(0x0)).0 as *mut u8
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}

pub unsafe fn init_memory_system() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());
    PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);
    println!("step 1");
    PAGE_DIRECTORY.map_range_addr(VirtualAddr(0), PhysicalAddr(0), ((1 << 20) * 128) >> 12).unwrap();
    println!("step 2");

    PAGE_DIRECTORY.map_range_addr(VirtualAddr(0xc0000000), PhysicalAddr(0xc0000000), ((1 << 20) * 1024) >> 12).unwrap();

    // for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
    //     dir_entry.set_present(true);
    //     debug_assert!(dir_entry.present() == true);
    // }

    println!("before enable paging");
    _enable_paging_with_cr(PAGE_DIRECTORY.as_mut().as_mut_ptr());

    println!("after enable paging");
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

/*

static mut PHYSICAL_BUDDIES: [u8; (((1024 * 1024 * 1024) / 4096) * 2 - 1) / 4 + 1] =
    [0u8; ((1024 * 1024 * 1024 / 4096) * 2 - 1) / 4 + 1];

pub static mut PHYSICAL_ALLOCATOR: Option<BuddyAllocator<'static, PhysicalAddr>> = None;

static mut VIRTUAL_BUDDIES: [u8; (((usize::max_value()) / 4096) * 2 - 1) / 4 + 1] =
    [0u8; ((usize::max_value() / 4096) * 2 - 1) / 4 + 1];

pub static mut VIRTUAL_ALLOCATOR: Option<BuddyAllocator<'static, VirtualAddr>> = None;

pub static mut PHYSICAL_DUMMY_ALLOCATOR_DATA: [u8; 1024 * 1024 * 16] = [0u8; 1024 * 1024 * 16];
pub static mut VIRTUAL_DUMMY_ALLOCATOR_DATA: [u8; 1024 * 1024 * 16] = [0u8; 1024 * 1024 * 16];

pub static mut PHYSICAL_DUMMY_ALLOCATOR: Option<DummyAllocator<'static>> = None;
pub static mut VIRTUAL_DUMMY_ALLOCATOR: Option<DummyAllocator<'static>> = None;

// pub static mut PHYSICAL_TEST: Option<[(DummyAllocator<'static>, PhysicalAllocatorType); 1]> = None;
// pub static mut VIRTUAL_TEST: Option<[(DummyAllocator<'static>, VirtualAllocatorType); 1]> = None;
pub static mut PHYSICAL_TEST: Option<[(BuddyAllocator<'static, PhysicalAddr>, PhysicalAllocatorType); 1]> = None;
pub static mut VIRTUAL_TEST: Option<[(BuddyAllocator<'static, VirtualAddr>, VirtualAllocatorType); 1]> = None;

pub static mut PAGE_ALLOCATOR: Option<PageAllocator<'static, 'static>> = None;

pub unsafe fn init_paging() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());

    PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);

    for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
        dir_entry.set_present(true);
        debug_assert!(dir_entry.present() == true);
    }

    PHYSICAL_ALLOCATOR = Some(BuddyAllocator::new(
        0x0,
        crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12,
        &mut PHYSICAL_BUDDIES,
    ));
    VIRTUAL_ALLOCATOR = Some(BuddyAllocator::new(0x0, 1024 * 1024, &mut VIRTUAL_BUDDIES));

    PHYSICAL_TEST = Some([(PHYSICAL_ALLOCATOR.take().unwrap(), PhysicalAllocatorType::Normal)]);
    VIRTUAL_TEST = Some([(VIRTUAL_ALLOCATOR.take().unwrap(), VirtualAllocatorType::KernelSpace)]);

    // PHYSICAL_DUMMY_ALLOCATOR = Some(DummyAllocator::new(
    //     0x0,
    //     crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12,
    //     PAGE_SIZE,
    //     &mut PHYSICAL_DUMMY_ALLOCATOR_DATA,
    // ));
    // VIRTUAL_DUMMY_ALLOCATOR = Some(DummyAllocator::new(
    //     0x0,
    //     crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12,
    //     PAGE_SIZE,
    //     &mut VIRTUAL_DUMMY_ALLOCATOR_DATA,
    // ));

    // PHYSICAL_TEST = Some([(PHYSICAL_DUMMY_ALLOCATOR.take().unwrap(), PhysicalAllocatorType::Normal)]);
    // VIRTUAL_TEST = Some([(VIRTUAL_DUMMY_ALLOCATOR.take().unwrap(), VirtualAllocatorType::KernelSpace)]);

    PAGE_ALLOCATOR = Some(PageAllocator::new(PHYSICAL_TEST.as_mut().unwrap(), VIRTUAL_TEST.as_mut().unwrap()));

    print_section!(text);
    print_section!(boot);
    print_section!(bss);
    print_section!(rodata);
    print_section!(data);
    print_section!(debug);

    for (_pd_index, table) in PAGE_TABLES.iter().enumerate() {
        for (_pt_index, entry) in table.as_ref().iter().enumerate() {
            // println!("{}:{} is_present: {}", pd_index, pt_index, entry.present());
            assert!(entry.present() == false);
            assert!(entry.inner == 0);
        }
    }

    for (section_name, (start_addr, end_addr)) in sections!() {
        assert!(start_addr <= end_addr);
        let section_size = end_addr - start_addr;
        //TODO: fonction de conversion
        println!("Remapping section {} to [{:x}:{:x}[", section_name, *start_addr, *start_addr + section_size);

        for p in (*start_addr..*end_addr).step_by(PAGE_SIZE) {
            // println!("Reserving addr {:x}", p);
            PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .reserve(VirtualAddr(p), PhysicalAddr(p), 1, AllocFlags(ALLOC_NORMAL))
                .expect("self kernel reserve failed");
        }
    }

    for page in Page::inclusive_range(Page::from_address(VirtualAddr(0x0)), Page::from_address(VirtualAddr(0x1000000)))
    {
        println!("{:?}", page);
    }
    loop {}
    // PAGE_ALLOCATOR
    //     .as_mut()
    //     .unwrap()
    //     .reserve(4244635648, 4244635648, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
    //     .expect("linear_frame_buffer mapping failed");

    // PAGE_ALLOCATOR
    //     .as_mut()
    //     .unwrap()
    //     .reserve(0x3000000, 0x3000000, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
    //     .expect("double frame buffer mapping failed");

    // PAGE_ALLOCATOR
    //     .as_mut()
    //     .unwrap()
    //     .reserve(0x4000000, 0x4000000, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
    //     .expect("graphic frame buffer mapping failed");

    // PAGE_ALLOCATOR
    //     .as_mut()
    //     .unwrap()
    //     .reserve(0x0, 0x0, (1024 * 1024) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
    //     .expect("Failed to reserve the first megabyte of physical memory");

    // println!("Enabling paging at {:p}", PAGE_DIRECTORY.as_mut().as_mut_ptr());
    // _enable_paging_with_cr(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    // Ok(())
}

pub struct MemoryManager;

impl MemoryManager {
    // pub unsafe fn init(&self) {
    //     VIRTUAL_ALLOCATOR = Some(BuddyAllocator::new(
    //         0x0,
    //         (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12) << 12,
    //         PAGE_SIZE,
    //         &mut VIRTUAL_BUDDIES,
    //     ));

    //     PHYSICAL_ALLOCATOR = Some(BuddyAllocator::new(
    //         0x0,
    //         (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12) << 12,
    //         PAGE_SIZE,
    //         &mut PHYSICAL_BUDDIES,
    //     ));
    // }
}

use core::alloc::{GlobalAlloc, Layout};

unsafe impl GlobalAlloc for MemoryManager {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().next_power_of_two();
        let page_allocator = PAGE_ALLOCATOR.as_mut().unwrap();

        let nbr_pages = size / 4096 + (size % 4096 != 0) as usize;

        page_allocator.alloc(nbr_pages, AllocFlags(ALLOC_NORMAL)).unwrap_or(VirtualAddr(0x0)).0 as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitfield() {
        let mut entry: PageDirectoryEntry = PageDirectoryEntry::new();

        assert_eq!(entry.present(), false);
        entry.set_present(true);
        assert_ne!(*entry.set_present(true), PageDirectoryEntry::new());
        assert_eq!(entry.present(), true);
    }
}
