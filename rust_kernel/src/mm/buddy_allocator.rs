use super::MemoryError;
use super::PAGE_SIZE;
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

    pub fn from_nbr_pages(nb_pages: usize) -> Self {
        Order(nb_pages.next_power_of_two().trailing_zeros() as usize)
    }

    pub fn from_size(size: usize) -> Self {
        Self::from_nbr_pages(size / PAGE_SIZE + (size % PAGE_SIZE != 0) as usize)
    }
}

pub struct BuddyAllocator<'a> {
    addr: usize,
    size: usize,
    max_order: Order,
    /// Invariant: all unused buddies are zeroed
    pub buddies: &'a mut [u8],
    nbr_buddies: usize,
}

impl<'a> BuddyAllocator<'a> {
    /// size in bytes
    pub fn new(addr: usize, size: usize, buddies: &'a mut [u8]) -> Self {
        assert!(addr % PAGE_SIZE == 0);

        let normalized_size = size.next_power_of_two();
        let unavailable_range = addr + size..addr + normalized_size;
        let max_order = Order::from_size(size);
        let nbr_buddies = Self::nbr_buddies(max_order.0);

        assert!(buddies.len() * 4 >= nbr_buddies);

        for byte in buddies.iter_mut() {
            *byte = 0;
        }
        dbg!(max_order);
        dbg!(nbr_buddies);

        let buddies_len = buddies.len();
        let mut new = BuddyAllocator { addr, size, max_order, buddies, nbr_buddies };

        for p in unavailable_range.step_by(PAGE_SIZE) {
            new.reserve(p, Order(0)).expect("exess memory reserved failed");
        }
        new
    }

    /// Returns the index of the buddy of order `order` starting at address `addr`.
    pub fn buddy_index(&self, addr: usize, order: Order) -> usize {
        assert_eq!(addr % (order.nbr_pages() * PAGE_SIZE), 0);
        assert!(addr >= self.addr && addr < self.addr + self.max_order.nbr_pages() * PAGE_SIZE);

        self.first_layer_index(order) + (addr - self.addr) / (order.nbr_pages() * PAGE_SIZE) 
    }

    // /// Returns the starting address represented by the buddy at index `index`.
    // fn buddy_addr(&self, index: usize) {
    //     let layer_index = Self::first_layer_index(order);
    //     let buddy_layer_index: usize = buddy_index - layer_index;
    //     self.max_order.nbr_pages() - ((index.next_power_of_two() >> 1 - 1) & index)
    // }

    /// size in bytes
    pub fn alloc(&mut self, order: Order) -> Option<usize> {
        if order > self.max_order {
            return None;
        }

        self.find_allocable_buddy(order).map(|buddy_index| {
            dbg!(buddy_index);
            self.set_occupied(buddy_index, true);
            let layer_index = self.first_layer_index(order);
            dbg!(layer_index);
            assert!(layer_index <= buddy_index);
            let buddy_layer_index: usize = buddy_index - layer_index;
            dbg!(buddy_layer_index);

            order.nbr_pages() * PAGE_SIZE * buddy_layer_index + self.addr
        }
    }

    pub fn free(&mut self, addr: usize, order: Order) {
        self.set_occupied(self.buddy_index(addr, order), false)
    }

    /// Reserves a buddy of order `order` starting at address `addr`.
    /// addr must be 
    pub fn reserve(&mut self, addr: usize, order: Order) -> Result<(), MemoryError> {
        assert_eq!(addr % (order.nbr_pages() * PAGE_SIZE), 0);

        if order > self.max_order || addr < self.addr || addr + order.nbr_pages() * PAGE_SIZE > self.addr + self.max_order.nbr_pages() * PAGE_SIZE {
            return Err(MemoryError::OutOfBound);
        }

        let index = self.buddy_index(addr, order);
        if self.get_buddy(index).occupied() {
            return Err(MemoryError::AlreadyOccupied);
        }

        self.set_occupied(index, true);
        Ok(())
    }

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
    }

    fn _find_allocable_buddy(&mut self, target_depth: usize, current_depth: usize, index: usize) -> Option<usize> {
        //dbg!(index);
        //dbg!(current_depth);
        if target_depth == current_depth {
            if self.get_buddy(index).occupied() {
                return None;
            }
            return Some(index);
        }

        if self.get_buddy(index).occupied() {
            return None;
        }

        if self.get_buddy(index).splitted() {
            let left_index = BuddyAllocator::left_child_index(index);
            let right_index = BuddyAllocator::right_child_index(index);

            if let Some(buddy_index) = self._find_allocable_buddy(target_depth, current_depth + 1, left_index) {
                return Some(buddy_index);
            }
            self._find_allocable_buddy(target_depth, current_depth + 1, right_index)
        } else {
            //            println!("Splitting buddy: {}", index);
            self.split_buddy(index);
            self._find_allocable_buddy(target_depth, current_depth + 1, BuddyAllocator::left_child_index(index))
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

        while let Some(parent_index) = BuddyAllocator::parent_index(index) {
            let left_child = BuddyAllocator::left_child_index(parent_index);
            let right_child = BuddyAllocator::right_child_index(parent_index);

            if self.get_buddy(right_child).occupied() == value && self.get_buddy(left_child).occupied() == value {
                self.get_buddy(parent_index).set_occupied(value);
                if value == false {
                    self.get_buddy(parent_index).set_splitted(false);
                }
            }

            // I think this should be like this...
            // if buddy is orphelin, set parent splitted recusivly
            if value == true {
                self.get_buddy(parent_index).set_splitted(true);
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

    /// Returns the size in bytes taken by the metadata taken by a BuddyAllocator of order `order`.
    pub fn metadata_size(max_order: Order) -> usize {
        Self::nbr_buddies(max_order.0) / 4 + 1
    }

    /// Returns the number of buddies of a BuddyAllocator of max order `order`.
    pub fn nbr_buddies(depth: usize) -> usize {
        2_usize.pow(depth as u32) * 2 - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::ffi::c_void;
    #[test]
    fn sodo_allocator() {
        use rand::prelude::*;
        use std::alloc::{Alloc, Global, Layout, System};

        const NB_ALLOC: usize = 1000;
        let mut allocator: System = System;

        const nb_block: usize = 0x10000;
        let address_space =
            unsafe { allocator.alloc(Layout::from_size_align(nb_block * PAGE_SIZE, PAGE_SIZE).unwrap()).unwrap() };
        const max_order: usize = nb_block.trailing_zeros() as usize;

        static mut BUDDIES: [u8; (nb_block * 2 - 1) / 4 + 1] = [0u8; (nb_block * 2 - 1) / 4 + 1];

        let mut buddy_allocator = unsafe {
            BuddyAllocator::new(address_space.as_ptr() as usize, nb_block * PAGE_SIZE, &mut BUDDIES)
        };

        #[derive(Debug)]
        struct Allocation<'a> {
            order: Order,
            random_u8: u8,
            ptr: &'a mut [u8],
        }

        let mut rng: StdRng = StdRng::seed_from_u64(4);

        let mut allocations: Vec<Allocation> = vec![];

        for nth_alloc in 0..NB_ALLOC {
            let type_alloc = rng.gen::<u32>() % 2;
            match type_alloc {
                0 => {
                    let order = Order(rng.gen::<usize>() % (max_order / 2));
                    let nb_page = 1 << order.0;
                    let mem = buddy_allocator.alloc(order);
                    dbg!(order);
                    dbg!(nb_page);
                    dbg!(mem);
                    // let mem = unsafe {
                    //     Some(
                    //         allocator
                    //             .alloc(Layout::from_size_align(nb_page * PAGE_SIZE, PAGE_SIZE).unwrap())
                    //             .unwrap()
                    //             .as_ptr() as usize,
                    //     )
                    // };
                    match mem {
                        None => {}
                        Some(mem) => {
                            let mem = unsafe { core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE) };
                            let random_u8 = rng.gen::<u8>();
                            for c in mem.iter_mut() {
                                *c = random_u8;
                            }
                            allocations.push(Allocation { order, ptr: mem, random_u8 });
                        }
                    }
                }
                1 => {
                    if allocations.len() != 0 {
                        println!("desaloc");
                        let index = rng.gen::<usize>() % allocations.len();
                        let elem = allocations.remove(index);
                        for (i, c) in elem.ptr.iter().enumerate() {
                            if *c != elem.random_u8 {
                                dbg!(index);
                                dbg!(i);
                                dbg!(nth_alloc);
                                assert_eq!(*c, elem.random_u8);
                            }
                        }
                        dbg!(elem.ptr.as_ptr() as usize);
                        dbg!( elem.order);
                        buddy_allocator.free(elem.ptr.as_ptr() as usize, elem.order);

                        // unsafe {
                        //     allocator.dealloc(
                        //         std::ptr::NonNull::new(elem.ptr.as_ptr() as *mut u8).unwrap(),
                        //         Layout::from_size_align(elem.nb_page * PAGE_SIZE, PAGE_SIZE).unwrap(),
                        //     )
                        // }
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
        static mut BUDDIES: [u8; (((1024 * 1024 * 1024) / 4096) * 2 - 1) / 8] =
            [0u8; ((1024 * 1024 * 1024 / 4096) * 2 - 1) / 8];

        let map_location = 0x00000000 as *const u8;
        let nb_block = 4;

        let mut buddy_allocator =
            unsafe { BuddyAllocator::new(map_location as usize, nb_block * 4096, &mut BUDDIES) };

        for i in 0..(nb_block) {
            let alloc_size = 1;
            let mut addr = buddy_allocator.alloc(Order(0));
            dbg!(i);
            assert_eq!(addr, Some(4096 * i));
        }
    }
}
