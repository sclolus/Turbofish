use super::{Address, MemoryError, NbrPages, PAGE_SIZE};
use bit_field::BitField;
use core::fmt::Debug;
use core::ops::{Add, IndexMut, Sub};

pub struct Buddy<'a> {
    data: &'a mut u8,
    index: u8,
}

impl<'a> Debug for Buddy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "[occupied: {}, splitted:{}]", self.occupied(), self.splitted())
    }
}

impl<'a> Buddy<'a> {
    pub fn new(data: &'a mut u8, index: u8) -> Self {
        assert!(index < 4);
        Self { data, index }
    }

    pub fn splitted(&self) -> bool {
        self.data.get_bit((self.index << 1) as usize)
    }

    pub fn set_splitted(&mut self, value: bool) -> &mut Self {
        self.data.set_bit((self.index << 1) as usize, value);
        self
    }

    pub fn occupied(&self) -> bool {
        self.data.get_bit((self.index << 1) as usize + 1)
    }

    pub fn set_occupied(&mut self, value: bool) -> &mut Self {
        self.data.set_bit((self.index << 1) as usize + 1, value);
        self
    }
}

/// 0 -> PAGE_SIZE bytes
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct Order(pub usize);

impl Sub<Self> for Order {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(rhs <= self);
        Order(self.0 - rhs.0)
    }
}

impl Add<Self> for Order {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Order(self.0 + rhs.0)
    }
}

impl Order {
    pub fn nbr_pages(self) -> usize {
        1 << self.0
    }

    #[inline(always)]
    pub fn from_nbr_pages(nbr_pages: usize) -> Self {
        Order(nbr_pages.next_power_of_two().trailing_zeros() as usize)
    }

    #[inline(always)]
    pub fn from_size(size: usize) -> Self {
        Self::from_nbr_pages((size >> 12) + (size & 0xFFF != 0) as usize)
    }
}

impl From<usize> for Order {
    fn from(nbr_pages: usize) -> Self {
        Order::from_nbr_pages(nbr_pages)
    }
}

/// METADATA for a buddy wich address 2 ^ 20 pages (ie: 4GB)
/// multiply by 2 to get nb buddies
/// divide by 4 because a buddy is 2 bytes
const METADATA_SIZE: usize = (1 << 20) * 2 / 4;

pub struct BuddyAllocator<T: Address> {
    addr: usize,
    /// In number of pages.
    size: usize,
    max_order: Order,
    /// Invariant: all unused buddies are zeroed
    buddies: [u8; METADATA_SIZE],
    nbr_buddies: usize,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Address> BuddyAllocator<T> {
    /// Size in number of pages.
    #[inline(always)]
    pub fn new(addr: usize, size: NbrPages) -> Self {
        let size = size.0;
        assert!(addr % PAGE_SIZE == 0);

        let normalized_size = size.next_power_of_two();
        let unavailable_range = size..normalized_size;
        let max_order = Order::from_nbr_pages(normalized_size);
        let nbr_buddies: usize = Self::nbr_buddies(max_order.0);

        let mut new = Self {
            addr,
            size,
            max_order,
            buddies: [0; METADATA_SIZE],
            nbr_buddies,
            _phantom: core::marker::PhantomData,
        };

        // Otim needed: reserve only one buddy if > size
        for page_offset in unavailable_range {
            new.reserve((addr + page_offset * PAGE_SIZE).into(), Order(0)).expect("exess memory reserved failed");
        }
        new
    }

    /// Returns the index of the buddy of order `order` starting at address `addr`.
    pub fn buddy_index(&self, addr: usize, order: Order) -> usize {
        assert!(addr >= self.addr && (addr - self.addr) / PAGE_SIZE < self.max_order.nbr_pages());
        assert_eq!((addr - self.addr) % (order.nbr_pages() * PAGE_SIZE), 0);

        self.first_layer_index(order) + (addr - self.addr) / (order.nbr_pages() * PAGE_SIZE)
    }

    /// size in bytes
    pub fn alloc(&mut self, order: Order) -> Result<T, MemoryError> {
        if order > self.max_order {
            return Err(MemoryError::OutOfBound);
        }

        match self.find_allocable_buddy(order) {
            Some(buddy_index) => {
                // dbg!(buddy_index);
                self.set_occupied(buddy_index, true);
                let layer_index = self.first_layer_index(order);
                // dbg!(layer_index);
                assert!(layer_index <= buddy_index);
                let buddy_layer_index: usize = buddy_index - layer_index;
                // dbg!(buddy_layer_index);

                Ok((order.nbr_pages() * PAGE_SIZE * buddy_layer_index + self.addr).into())
            }
            None => Err(MemoryError::OutOfMem),
        }
    }

    pub fn free(&mut self, addr: T, order: Order) -> Result<(), MemoryError> {
        let addr: usize = addr.into();
        assert!((addr - self.addr) % PAGE_SIZE == 0);
        assert!((addr - self.addr) / PAGE_SIZE < self.size);

        let buddy_index = self.buddy_index(addr, order);

        if !self.get_buddy(buddy_index).occupied() || self.get_buddy(buddy_index).splitted() {
            Err(MemoryError::CannotFree)
        } else {
            self.set_occupied(buddy_index, false);
            Ok(())
        }
    }

    /// Reserves a buddy of order `order` starting at address `addr`.
    /// # Panic
    /// panic if addr is not a multiple of order.nbr_pages() * PAGE_SIZE
    pub fn reserve(&mut self, addr: T, order: Order) -> Result<(), MemoryError> {
        let addr: usize = addr.into();
        if order > self.max_order
            || addr < self.addr
            || (addr - self.addr) / PAGE_SIZE + order.nbr_pages() > self.max_order.nbr_pages()
        {
            return Err(MemoryError::OutOfBound);
        }
        assert_eq!((addr - self.addr) % (order.nbr_pages() * PAGE_SIZE), 0);

        let index = self.buddy_index(addr, order);
        if self.get_buddy(index).occupied() || self.get_buddy(index).splitted() {
            return Err(MemoryError::AlreadyOccupied);
        }

        let mut current_index = index;
        while let Some(parent_index) = Self::parent_index(current_index) {
            let parent_buddy = self.get_buddy(parent_index);
            if parent_buddy.occupied() {
                return Err(MemoryError::AlreadyOccupied);
            } else if parent_buddy.splitted() {
                break;
            }
            current_index = parent_index;
        }

        self.set_occupied(index, true);
        Ok(())
    }

    // pub fn reserve_range(&mut self, range: PageIter<T>) -> Result<(), MemoryError> {
    //     let start = range.current;
    //     for page in range {
    //         if let Err(_) = self.reserve(page.into_addr(), Order(0)) {
    //             for p in Page::exclusive_range(start, page) {
    //                 self.free(p.into(), Order(0));
    //             }
    //             return Err(MemoryError::AlreadyOccupied);
    //         }
    //     }
    //     Ok(())
    // }
    fn left_child_index(i: usize) -> usize {
        i * 2 + 1
    }

    fn right_child_index(i: usize) -> usize {
        i * 2 + 2
    }

    fn parent_index(i: usize) -> Option<usize> {
        Some(i.checked_sub(1)? / 2)
    }

    pub fn get_buddy(&mut self, index: usize) -> Buddy<'_> {
        Buddy::new(self.buddies.index_mut(index >> 2), index as u8 & 0b11)
    }

    fn split_buddy(&mut self, index: usize) {
        // dbg!(index);
        // dbg!(self.nbr_buddies);
        assert!(index < self.nbr_buddies / 2);
        assert!(self.get_buddy(index).splitted() == false);
        assert!(self.get_buddy(index).occupied() == false);

        self.get_buddy(index).set_splitted(true);
        assert!(self.get_buddy(index).splitted() == true);
    }

    fn _find_allocable_buddy(&mut self, target_depth: usize, current_depth: usize, index: usize) -> Option<usize> {
        if target_depth == current_depth {
            if self.get_buddy(index).occupied() || self.get_buddy(index).splitted() {
                return None;
            }
            return Some(index);
        }

        if self.get_buddy(index).occupied() {
            return None;
        }

        if self.get_buddy(index).splitted() {
            let left_index = Self::left_child_index(index);
            let right_index = Self::right_child_index(index);

            if let Some(buddy_index) = self._find_allocable_buddy(target_depth, current_depth + 1, left_index) {
                return Some(buddy_index);
            }
            self._find_allocable_buddy(target_depth, current_depth + 1, right_index)
        } else {
            //            println!("Splitting buddy: {}", index);
            self.split_buddy(index);
            self._find_allocable_buddy(target_depth, current_depth + 1, Self::left_child_index(index))
        }
    }

    fn find_allocable_buddy(&mut self, order: Order) -> Option<usize> {
        self._find_allocable_buddy(self.depth_buddy_from_order(order), 0, 0)
    }

    fn set_occupied(&mut self, mut index: usize, value: bool) {
        self.get_buddy(index).set_occupied(value);
        if value == false {
            self.get_buddy(index).set_splitted(false);
        }

        while let Some(parent_index) = Self::parent_index(index) {
            let left_child = Self::left_child_index(parent_index);
            let right_child = Self::right_child_index(parent_index);

            if self.get_buddy(right_child).occupied() == value && self.get_buddy(left_child).occupied() == value {
                self.get_buddy(parent_index).set_occupied(value);
                if value == false
                    && self.get_buddy(right_child).splitted() == false
                    && self.get_buddy(left_child).splitted() == false
                {
                    self.get_buddy(parent_index).set_splitted(false);
                }
            }

            // if buddy is orphelin, set parent splitted recusivly
            if value == true {
                self.get_buddy(parent_index).set_splitted(true);
                assert!(self.get_buddy(parent_index).splitted() == true);
            }

            index = parent_index;
        }
    }

    /// size in number of pages.
    fn depth_buddy_from_order(&self, order: Order) -> usize {
        self.max_order.0 - order.0
    }

    /// Returns the Buddyallocator's first index (as by layer indexing of perfect Btree) at the layer of buddies of order `order`.
    fn first_layer_index(&self, order: Order) -> usize {
        if order == self.max_order {
            return 0;
        }
        Self::nbr_buddies(self.depth_buddy_from_order(order) - 1)
    }

    /// Returns the number of buddies of a BuddyAllocator of max order `order`.
    pub fn nbr_buddies(depth: usize) -> usize {
        2_usize.pow(depth as u32) * 2 - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mm::VirtualAddr;
    use core::ffi::c_void;
    #[test]
    fn sodo_allocator() {
        use rand::prelude::*;
        use std::alloc::{Alloc, Global, Layout, System};

        const NB_ALLOC: usize = 100;
        let mut allocator: System = System;

        const NB_BLOCK: usize = 0x10000;
        let address_space =
            unsafe { allocator.alloc(Layout::from_size_align(NB_BLOCK * PAGE_SIZE, PAGE_SIZE).unwrap()).unwrap() };
        const MAX_ORDER: usize = NB_BLOCK.trailing_zeros() as usize;

        let mut buddy_allocator: BuddyAllocator<VirtualAddr> =
            unsafe { BuddyAllocator::new(address_space.as_ptr() as usize, NbrPages(NB_BLOCK)) };

        #[derive(Debug)]
        struct Allocation<'a> {
            order: Order,
            buddy_index: usize,
            random_u8: u8,
            ptr: &'a mut [u8],
        }
        use fmt::{Display, Formatter};
        use std::fmt;

        impl<'a> Display for Allocation<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                let ptr = self.ptr as *const _ as *const u8 as usize;
                write!(
                    f,
                    "[{:x}:{:x}[, order: {}, random_byte: {:x}",
                    ptr,
                    ptr + self.order.nbr_pages() * PAGE_SIZE,
                    self.order.0,
                    self.random_u8
                )
            }
        }

        let mut rng: StdRng = StdRng::seed_from_u64(4);

        let mut allocations: Vec<Allocation> = vec![];

        for _nth_alloc in 0..NB_ALLOC {
            let type_alloc = rng.gen::<u32>() % 3;
            match type_alloc {
                0 => {
                    let order = Order(rng.gen::<usize>() % (MAX_ORDER / 2));
                    let nb_page = 1 << order.0;

                    //                        eprintln!("Attempting to allocate a region of order {} (nbr_pages: {})", order.0, order.nbr_pages());
                    let mem = buddy_allocator.alloc(order);
                    // let mem = unsafe {
                    //     Some(
                    //         allocator
                    //             .alloc(Layout::from_size_align(nb_page * PAGE_SIZE, PAGE_SIZE).unwrap())
                    //             .unwrap()
                    //             .as_ptr() as usize,
                    //     )
                    // };
                    match mem {
                        Err(e) => eprintln!("Failed to allocate {:?}", e),
                        Ok(VirtualAddr(mem)) => {
                            let mem = unsafe { core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE) };
                            let random_u8 = rng.gen::<u8>();
                            for c in mem.iter_mut() {
                                *c = random_u8;
                            }
                            let elem = Allocation {
                                order,
                                buddy_index: buddy_allocator.buddy_index(mem as *const _ as *const u8 as usize, order),
                                ptr: mem,
                                random_u8,
                            };
                            //                                eprintln!("Got {}\n", elem);
                            allocations.push(elem);
                        }
                    }
                }
                1 => {
                    if allocations.len() != 0 {
                        let index = rng.gen::<usize>() % allocations.len();
                        let elem = allocations.remove(index);
                        //                            eprintln!("Attempting to free {}", elem);
                        assert_eq!(
                            elem.buddy_index,
                            buddy_allocator.buddy_index(elem.ptr as *const _ as *const u8 as usize, elem.order)
                        );
                        buddy_allocator.free(VirtualAddr(elem.ptr.as_ptr() as usize), elem.order);
                        for (_i, c) in elem.ptr.iter().enumerate() {
                            if *c != elem.random_u8 {
                                println!("{} has erroneous byte {:x} at {:p}", elem, *c, c);
                                println!("Allocations matching byte {:x}: ", *c);
                                for matching in allocations.iter().filter(|x| x.random_u8 == *c) {
                                    eprintln!(" {}", matching);
                                }

                                assert_eq!(*c, elem.random_u8);
                            }
                        }
                        // buddy_allocator.free(elem.ptr.as_ptr() as usize, elem.order);

                        //                            eprintln!("");

                        // unsafe {
                        //     allocator.dealloc(
                        //         std::ptr::NonNull::new(elem.ptr.as_ptr() as *mut u8).unwrap(),
                        //         Layout::from_size_align(elem.nb_page * PAGE_SIZE, PAGE_SIZE).unwrap(),
                        //     )
                        // }
                    }
                }
                2 => {
                    let order = Order(rng.gen::<usize>() % (MAX_ORDER / 2));
                    let rand_max = (NB_BLOCK * PAGE_SIZE) / (order.nbr_pages() * PAGE_SIZE);
                    let addr = address_space.as_ptr() as usize
                        + (rng.gen::<usize>() % rand_max) * order.nbr_pages() * PAGE_SIZE;

                    let nb_page = 1 << order.0;

                    //                        eprintln!("Attempting to reserve a region [{:x}:{:x}[ of order {} (nbr_pages: {})", addr, addr + order.nbr_pages() * PAGE_SIZE, order.0, order.nbr_pages());
                    let mem = buddy_allocator.reserve(VirtualAddr(addr), order);
                    match mem {
                        Err(err) => eprintln!("Failed to reserve: {:?}", err),
                        Ok(_) => {
                            let mem = addr;
                            let mem = unsafe { core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE) };
                            let random_u8 = rng.gen::<u8>();
                            for c in mem.iter_mut() {
                                *c = random_u8;
                            }
                            let elem = Allocation {
                                order,
                                buddy_index: buddy_allocator.buddy_index(mem as *const _ as *const u8 as usize, order),
                                ptr: mem,
                                random_u8,
                            };
                            //                                eprintln!("Got {}\n", elem);
                            allocations.push(elem);
                        }
                    }
                }
                _ => {
                    panic!("WTF");
                }
            }
        }
    }

    #[test]
    fn test_allocator() {
        const NB_BLOCK: usize = 4;
        let map_location = 0x00010000 as *const u8;

        let mut buddy_allocator: BuddyAllocator<VirtualAddr> =
            unsafe { BuddyAllocator::new(map_location as usize, NbrPages(NB_BLOCK)) };

        let alloc_size = 1;
        for i in 0..(NB_BLOCK) {
            let addr = buddy_allocator.alloc(alloc_size.into());
            dbg!(i);
            assert_eq!(addr, Ok(VirtualAddr(map_location as usize + PAGE_SIZE * i)));
        }
        for i in 0..(NB_BLOCK) {
            buddy_allocator.free(VirtualAddr(map_location as usize + PAGE_SIZE * i), alloc_size.into());
        }
    }
}
