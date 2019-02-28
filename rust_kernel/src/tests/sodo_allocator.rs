use crate::debug;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, PIC_8259};
use crate::io::{Pio, UART_16550};
use crate::mm;
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};
use crate::tests::helpers::exit_qemu;
use crate::timer::Rtc;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
    fn _get_esp() -> u32;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) -> u32 {
    save_multiboot_info(multiboot_info);
    unsafe {
        println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
        println!("base memory: {:?} {:?}", MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);
    }

    unsafe {
        interrupts::init();
    }
    unsafe {
        println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
    }
    unsafe {
        eprintln!("{:08x}", _get_esp());
    }
    unsafe {
        mm::init_memory_system().unwrap();
    }
    println!("bonjour");
    exit_qemu(0);
    0
    // use rand::prelude::*;
    // use std::alloc::{Alloc, Global, Layout, System};

    // const NB_ALLOC: usize = 100;
    // let mut allocator: System = System;

    // const NB_BLOCK: usize = 0x10000;
    // let address_space =
    //     unsafe { allocator.alloc(Layout::from_size_align(NB_BLOCK * PAGE_SIZE, PAGE_SIZE).unwrap()).unwrap() };
    // const MAX_ORDER: usize = NB_BLOCK.trailing_zeros() as usize;

    // let mut buddy_allocator: BuddyAllocator<VirtualAddr> =
    //     unsafe { BuddyAllocator::new(address_space.as_ptr() as usize, NbrPages(NB_BLOCK)) };

    // #[derive(Debug)]
    // struct Allocation<'a> {
    //     order: Order,
    //     buddy_index: usize,
    //     random_u8: u8,
    //     ptr: &'a mut [u8],
    // }
    // use fmt::{Display, Formatter};
    // use std::fmt;

    // impl<'a> Display for Allocation<'a> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
    //         let ptr = self.ptr as *const _ as *const u8 as usize;
    //         write!(
    //             f,
    //             "[{:x}:{:x}[, order: {}, random_byte: {:x}",
    //             ptr,
    //             ptr + self.order.nbr_pages() * PAGE_SIZE,
    //             self.order.0,
    //             self.random_u8
    //         )
    //     }
    // }

    // let mut rng: StdRng = StdRng::seed_from_u64(4);

    // let mut allocations: Vec<Allocation> = vec![];

    // for _nth_alloc in 0..NB_ALLOC {
    //     let type_alloc = rng.gen::<u32>() % 3;
    //     match type_alloc {
    //         0 => {
    //             let order = Order(rng.gen::<usize>() % (MAX_ORDER / 2));
    //             let nb_page = 1 << order.0;

    //             //                        eprintln!("Attempting to allocate a region of order {} (nbr_pages: {})", order.0, order.nbr_pages());
    //             let mem = buddy_allocator.alloc(order);
    //             // let mem = unsafe {
    //             //     Some(
    //             //         allocator
    //             //             .alloc(Layout::from_size_align(nb_page * PAGE_SIZE, PAGE_SIZE).unwrap())
    //             //             .unwrap()
    //             //             .as_ptr() as usize,
    //             //     )
    //             // };
    //             match mem {
    //                 Err(e) => eprintln!("Failed to allocate {:?}", e),
    //                 Ok(VirtualAddr(mem)) => {
    //                     let mem = unsafe { core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE) };
    //                     let random_u8 = rng.gen::<u8>();
    //                     for c in mem.iter_mut() {
    //                         *c = random_u8;
    //                     }
    //                     let elem = Allocation {
    //                         order,
    //                         buddy_index: buddy_allocator.buddy_index(mem as *const _ as *const u8 as usize, order),
    //                         ptr: mem,
    //                         random_u8,
    //                     };
    //                     //                                eprintln!("Got {}\n", elem);
    //                     allocations.push(elem);
    //                 }
    //             }
    //         }
    //         1 => {
    //             if allocations.len() != 0 {
    //                 let index = rng.gen::<usize>() % allocations.len();
    //                 let elem = allocations.remove(index);
    //                 //                            eprintln!("Attempting to free {}", elem);
    //                 assert_eq!(
    //                     elem.buddy_index,
    //                     buddy_allocator.buddy_index(elem.ptr as *const _ as *const u8 as usize, elem.order)
    //                 );
    //                 buddy_allocator.free(VirtualAddr(elem.ptr.as_ptr() as usize), elem.order);
    //                 for (_i, c) in elem.ptr.iter().enumerate() {
    //                     if *c != elem.random_u8 {
    //                         println!("{} has erroneous byte {:x} at {:p}", elem, *c, c);
    //                         println!("Allocations matching byte {:x}: ", *c);
    //                         for matching in allocations.iter().filter(|x| x.random_u8 == *c) {
    //                             eprintln!(" {}", matching);
    //                         }

    //                         assert_eq!(*c, elem.random_u8);
    //                     }
    //                 }
    //                 // buddy_allocator.free(elem.ptr.as_ptr() as usize, elem.order);

    //                 //                            eprintln!("");

    //                 // unsafe {
    //                 //     allocator.dealloc(
    //                 //         std::ptr::NonNull::new(elem.ptr.as_ptr() as *mut u8).unwrap(),
    //                 //         Layout::from_size_align(elem.nb_page * PAGE_SIZE, PAGE_SIZE).unwrap(),
    //                 //     )
    //                 // }
    //             }
    //         }
    //         2 => {
    //             let order = Order(rng.gen::<usize>() % (MAX_ORDER / 2));
    //             let rand_max = (NB_BLOCK * PAGE_SIZE) / (order.nbr_pages() * PAGE_SIZE);
    //             let addr =
    //                 address_space.as_ptr() as usize + (rng.gen::<usize>() % rand_max) * order.nbr_pages() * PAGE_SIZE;

    //             let nb_page = 1 << order.0;

    //             //                        eprintln!("Attempting to reserve a region [{:x}:{:x}[ of order {} (nbr_pages: {})", addr, addr + order.nbr_pages() * PAGE_SIZE, order.0, order.nbr_pages());
    //             let mem = buddy_allocator.reserve(VirtualAddr(addr), order);
    //             match mem {
    //                 Err(err) => eprintln!("Failed to reserve: {:?}", err),
    //                 Ok(_) => {
    //                     let mem = addr;
    //                     let mem = unsafe { core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE) };
    //                     let random_u8 = rng.gen::<u8>();
    //                     for c in mem.iter_mut() {
    //                         *c = random_u8;
    //                     }
    //                     let elem = Allocation {
    //                         order,
    //                         buddy_index: buddy_allocator.buddy_index(mem as *const _ as *const u8 as usize, order),
    //                         ptr: mem,
    //                         random_u8,
    //                     };
    //                     //                                eprintln!("Got {}\n", elem);
    //                     allocations.push(elem);
    //                 }
    //             }
    //         }
    //         _ => {
    //             panic!("WTF");
    //         }
    //     }
    // }
}
