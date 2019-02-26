//! This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
//! See https://wiki.osdev.org/Paging for relevant documentation.
pub mod buddy_allocator;
pub mod page_alloc;
pub mod page_directory;
pub mod page_table;
use bit_field::BitField;
use buddy_allocator::*;
use core::convert::AsRef;
use core::ops::Range;
use page_alloc::{AllocFlags, PageAllocator, PhysicalAllocatorType, VirtualAllocatorType, ALLOC_NORMAL};

//Remove this eventually.
pub mod dummy_allocator;
use dummy_allocator::DummyAllocator;

pub const PAGE_SIZE: usize = 4096;
// const_assert!(PAGE_SIZE.is_power_of_two());

use page_directory::{PageDirectory, PageDirectoryEntry};
use page_table::{PageTable, PageTableEntry};

extern "C" {
    fn _enable_paging(addr: *mut PageDirectoryEntry);
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

#[derive(Debug, Copy, Clone)]
pub enum MemoryError {
    OutOfMem,
    OutOfBound,
    AlreadyOccupied,
    NotSatifiableFlags,
    AlreadyMapped,
}
macro_rules! print_section {
    ($ident: ident) => {
        println!(
            "{}: [{:p}: {:p}[",
            stringify!($ident),
            &concat_idents!(__, start_, $ident),
            &concat_idents!(__, end_, $ident)
        );
    };
}

macro_rules! get_section_tuple {
    ($ident: ident) => {
        (
            &concat_idents!(__, start_, $ident) as *const _ as usize,
            &concat_idents!(__, end_, $ident) as *const _ as usize,
        )
    };
}
macro_rules! sections {
    () => {
        [
            ("text", get_section_tuple!(text)),
            ("boot", get_section_tuple!(boot)),
            ("bss", get_section_tuple!(bss)),
            ("rodata", get_section_tuple!(rodata)),
            ("data", get_section_tuple!(data)),
            ("debug", get_section_tuple!(debug)),
        ]
        .iter()
    };
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtualAddr {
    addr: usize,
}

impl VirtualAddr {
    pub fn physical_addr(&self) -> Option<PhysicalAddr> {
        let page_directory_index = self.addr.get_bits(22..32);
        let page_table_index = self.addr.get_bits(12..22);

        unsafe {
            if PAGE_DIRECTORY[page_directory_index].present() {
                return None;
            }
        }

        // if PAGE_TABLES[
        None
    }

    pub fn pd_index(&self) -> usize {
        self.addr.get_bits(22..32)
    }

    pub fn pt_index(&self) -> usize {
        self.addr.get_bits(12..22)
    }

    pub fn offset(&self) -> usize {
        self.addr.get_bits(0..12)
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysicalAddr {
    addr: usize,
}

impl PhysicalAddr {}

#[allow(dead_code)]
static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

// static mut PHYSICAL_BUDDIES: [u8; (((1024 * 1024 * 1024) / 4096) * 2 - 1) / 8] =
//     [0u8; ((1024 * 1024 * 1024 / 4096) * 2 - 1) / 8];

// pub static mut PHYSICAL_ALLOCATOR: Option<BuddyAllocator<'static>> = None;

// static mut VIRTUAL_BUDDIES: [u8; (((1024 * 1024 * 1024) / 4096) * 2 - 1) / 8] =
//     [0u8; ((1024 * 1024 * 1024 / 4096) * 2 - 1) / 8];

// pub static mut VIRTUAL_ALLOCATOR: Option<BuddyAllocator<'static>> = None;

pub static mut PHYSICAL_DUMMY_ALLOCATOR_DATA: [u8; 1024 * 1024 * 16] = [0u8; 1024 * 1024 * 16];
pub static mut VIRTUAL_DUMMY_ALLOCATOR_DATA: [u8; 1024 * 1024 * 16] = [0u8; 1024 * 1024 * 16];

pub static mut PHYSICAL_DUMMY_ALLOCATOR: Option<DummyAllocator<'static>> = None;
pub static mut VIRTUAL_DUMMY_ALLOCATOR: Option<DummyAllocator<'static>> = None;

pub static mut PHYSICAL_TEST: Option<[(DummyAllocator<'static>, PhysicalAllocatorType); 1]> = None;
pub static mut VIRTUAL_TEST: Option<[(DummyAllocator<'static>, VirtualAllocatorType); 1]> = None;
pub static mut PAGE_ALLOCATOR: Option<PageAllocator<'static, 'static>> = None;

pub unsafe fn init_paging() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());

    PAGE_DIRECTORY.set_page_tables(0, &PAGE_TABLES);

    for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
        dir_entry.set_present(true);
        debug_assert!(dir_entry.present() == true);
    }
    //MEMORY_MANAGER.init();

    // PAGE_DIRECTORY.self_map_tables();

    // VIRTUAL_ALLOCATOR = Some(BuddyAllocator::new(
    //     0x0,
    //     (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12) << 12,
    //     PAGE_SIZE,
    //     &mut VIRTUAL_BUDDIES,
    // ));

    // PHYSICAL_ALLOCATOR = Some(BuddyAllocator::new(
    //     0x0,
    //     (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12) << 12,
    //     PAGE_SIZE,
    //     &mut PHYSICAL_BUDDIES,
    // ));

    PHYSICAL_DUMMY_ALLOCATOR = Some(DummyAllocator::new(
        0x0,
        (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12),
        PAGE_SIZE,
        &mut PHYSICAL_DUMMY_ALLOCATOR_DATA,
    ));
    VIRTUAL_DUMMY_ALLOCATOR = Some(DummyAllocator::new(
        0x0,
        (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12),
        PAGE_SIZE,
        &mut VIRTUAL_DUMMY_ALLOCATOR_DATA,
    ));

    PHYSICAL_TEST = Some([(PHYSICAL_DUMMY_ALLOCATOR.take().unwrap(), PhysicalAllocatorType::Normal)]);
    VIRTUAL_TEST = Some([(VIRTUAL_DUMMY_ALLOCATOR.take().unwrap(), VirtualAllocatorType::KernelSpace)]);

    PAGE_ALLOCATOR = Some(PageAllocator::new(PHYSICAL_TEST.as_mut().unwrap(), VIRTUAL_TEST.as_mut().unwrap()));
    //MEMORY_MANAGER.init();
    ///// reserve the first 8 megabytes. This is just a commodity for now.
    //let _layout = core::alloc::Layout::from_size_align(1024 * 1024 * 8, PAGE_SIZE).unwrap();
    //dbg!(MEMORY_MANAGER.alloc(_layout));

    print_section!(text);
    print_section!(boot);
    print_section!(bss);
    print_section!(rodata);
    print_section!(data);
    print_section!(debug);

    for (pd_index, table) in PAGE_TABLES.iter().enumerate() {
        for (pt_index, entry) in table.as_ref().iter().enumerate() {
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
                .reserve(p, p, 1, AllocFlags(ALLOC_NORMAL))
                .expect("self kernel reserve failed");
        }
    }

    PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .reserve(4244635648, 4244635648, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
        .expect("linear_frame_buffer mapping failed");

    PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .reserve(0x3000000, 0x3000000, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
        .expect("linear frame buffer mapping failed");

    PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .reserve(0x4000000, 0x4000000, (1024 * 768 * 3) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
        .expect("linear frame buffer mapping failed");

    PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .reserve(0x0, 0x0, (1024 * 1024) / PAGE_SIZE, AllocFlags(ALLOC_NORMAL))
        .expect("linear frame buffer mapping failed");

    _enable_paging(PAGE_DIRECTORY.as_mut().as_mut_ptr());
    Ok(())
}

pub struct MemoryManager;
use crate::MEMORY_MANAGER;

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
        use core::ptr;
        let size = layout.size().next_power_of_two();
        let page_allocator = PAGE_ALLOCATOR.as_mut().unwrap();

        let nbr_pages = size / 4096 + (size % 4096 != 0) as usize;

        page_allocator.alloc(nbr_pages, AllocFlags(ALLOC_NORMAL)).unwrap_or(0x0) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}

#[alloc_error_handler]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}

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
