pub mod page_alloc;
/// This module contains the code for the Memory Management Unit and (probably) the Current Implementation of the Memory Manager
/// See https://wiki.osdev.org/Paging for relevant documentation.
pub mod page_directory;
pub mod page_table;
use bit_field::BitField;
use core::ops::Range;

// pub use page_directory::PageDirectoryEntry;
// pub use page_table::PageTableEntry;

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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysicalAddr {
    addr: usize,
}

impl PhysicalAddr {}

#[allow(dead_code)]
static mut PAGE_TABLES: [PageTable; PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE] = // should be renamed to INIT_PAGE_TABLES
    [PageTable::new(); PageDirectory::DEFAULT_PAGE_DIRECTORY_SIZE];

static mut PAGE_DIRECTORY: PageDirectory = PageDirectory::new(); // Should be renamed to INIT_PAGE_DIRECTORY

static mut BUDDIES: [page_alloc::Buddy; ((1024 * 1024 * 1024) / 4096) * 2 - 1] =
    [page_alloc::Buddy::new(); (1024 * 1024 * 1024 / 4096) * 2 - 1];

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

    let map_location = 0x00000000 as *const u8;

    let mut buddy_allocator =
        page_alloc::BuddyAllocator::new(map_location as usize, 1024 * 1024 * 1024, 4096, &mut BUDDIES);
    println!("mapping [{:x}:{:x}[ to {:p}", start_addr, end_addr, map_location);
    // println!("mapping_addr: {:p}", buddy_allocator.alloc(4).unwrap());

    for __ in 0..(1024 * 1024 * 1024) {
        let alloc_size = 4096 * 1024;
        let mut addr = buddy_allocator.alloc(alloc_size);

        if addr.is_some() {
            let buddy_index = buddy_allocator.buddy_index(addr.unwrap() as usize, alloc_size);
        // println!("mapping_addr: {:?}, buddy_index: {}", addr, buddy_index);
        } else {
            break;
            // println!("mapping_addr: {:?}", addr);
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
