pub mod page_alloc;
/// This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
/// See https://wiki.osdev.org/Paging for relevant documentation.
pub mod page_directory;
pub mod page_table;
use bit_field::BitField;
use core::ops::Range;

pub const PAGE_SIZE: usize = 4096;

use page_directory::{PageDirectory, PageDirectoryEntry};
use page_table::{PageTable, PageTableEntry};

extern "C" {
    static __test_symbol: u8;
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

static mut BUDDIES: [u8; (((1024 * 1024 * 1024) / 4096) * 2 - 1) / 8] =
    [0u8; ((1024 * 1024 * 1024 / 4096) * 2 - 1) / 8];

pub static mut PHYSICAL_ALLOCATOR: Option<page_alloc::BuddyAllocator<'static>> = None;

pub unsafe fn init_paging() -> Result<(), ()> {
    println!("pointeur to page_directory: {:p}", PAGE_DIRECTORY.as_ref().as_ptr());

    for dir_entry in PAGE_DIRECTORY.as_mut().iter_mut() {
        dir_entry.set_present(false);
        debug_assert!(dir_entry.present() == false);
    }
    PAGE_DIRECTORY.self_map_tables();

    let first_directory_entry = *PageDirectoryEntry::new()
        .set_present(true)
        .set_read_write(true)
        .set_entry_addr((PAGE_TABLES[0].as_ref().as_ptr() as usize));

    println!("ptr for first entry: {:x}", first_directory_entry.entry_addr() << 12);
    PAGE_DIRECTORY[0] = first_directory_entry;

    let init_page_table = &mut PAGE_TABLES[0];
    for index in 0u32..1024u32 {
        let page_entry = *PageTableEntry::new()
            .set_global(true)
            .set_present(true)
            .set_read_write(true)
            .set_physical_address((index as usize) << 12);

        init_page_table[index as usize] = page_entry;
    }

    use crate::monitor::SCREEN_MONAD;
    println!("arg to enable_paging: {:p}", PAGE_DIRECTORY.as_mut().as_mut_ptr());
    // PAGE_DIRECTORY.auto_ref_kernel_base();

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

    println!("{:x}", __test_symbol);
    print_section!(text);
    print_section!(boot);
    print_section!(bss);
    print_section!(rodata);
    print_section!(debug);

    let (start_addr, end_addr) = get_section_tuple!(text);

    println!("mapping [{:x}:{:x}[ to {:p}", start_addr, end_addr, 0x0 as *const u8);
    PHYSICAL_ALLOCATOR = Some(unsafe {
        page_alloc::BuddyAllocator::new(
            0x0,
            (crate::multiboot::MULTIBOOT_INFO.unwrap().get_system_memory_amount() >> 12) << 12,
            4096,
            &mut BUDDIES,
        )
    }); // println!("mapping_addr: {:p}", buddy_allocator.alloc(4).unwrap());

    for __ in 0..(1024 * 1024 * 1024) {
        let alloc_size = 4096 * 1024;
        let mut addr = PHYSICAL_ALLOCATOR.as_mut().unwrap().alloc(alloc_size);

        if addr.is_some() {
            let buddy_index = PHYSICAL_ALLOCATOR.as_mut().unwrap().buddy_index(addr.unwrap() as usize, alloc_size);
            println!("mapping_addr: {:?}, buddy_index: {}", addr, buddy_index);
        } else {
            println!("mapping_addr: {:?}", addr);
            break;
        }
        // buddy_allocator.free(addr.unwrap() as usize, alloc_size);
    }
    println!("done");
    // dbg!(buddy_allocator.buddies);
    // println!("mapping_addr: {:?}", buddy_allocator.alloc(8192));
    // println!("mapping_addr: {:?}", buddy_allocator.alloc(4096));
    // println!("mapping_addr: {:?}", buddy_allocator.alloc(4096));
    // println!("mapping_addr: {:?}", buddy_allocator.alloc(4096));
    // println!("mapping_addr: {:?}", buddy_allocator.alloc(4096));

    loop {}
    // PAGE_DIRECTORY
    //     .remap_range_addr(4244635648..(4244635648 + 1024 * 768 * 3), 4244635648..(4244635648 + 1024 * 768 * 3));
    println!("{:?}", SCREEN_MONAD);
    _enable_paging(PAGE_DIRECTORY.as_mut().as_mut_ptr());

    Ok(())
}

extern "C" {
    fn _enable_paging(addr: *mut PageDirectoryEntry);
    fn _enable_pse();
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
