use crate::memory::tools::*;
use alloc::vec::Vec;
use bit_field::BitField;
use core::fmt;
use core::fmt::Debug;
use core::ops::{Add, IndexMut, Sub};
use fallible_collections::try_vec;
use try_clone_derive::TryClone;

#[derive(Debug, Clone, TryClone, PartialEq)]
pub struct BuddyAllocator<T: Address> {
    addr: Page<T>,
    /// In number of pages.
    size: NbrPages,
    max_order: Order,
    /// Invariant: all unused buddies are zeroed
    buddies: Vec<u8>,
    nbr_buddies: usize,
}

impl<T: Address> BuddyAllocator<T> {
    /// buddies must be a zeroed vec
    pub fn new(addr: Page<T>, size: NbrPages) -> Result<Self> {
        let max_order: Order = size.into();
        let nbr_buddies: usize = Self::nbr_buddies(max_order.0);

        let new = Self {
            addr,
            size,
            max_order,
            buddies: try_vec![0; BuddyAllocator::<Virt>::metadata_size(size)]
                .map_err(|_| MemoryError::OutOfMem)?,
            nbr_buddies,
        };

        // let normalized_size = size.0.next_power_of_two();
        // let unavailable_range = size.0..normalized_size;
        // TODO: Otim needed: reserve only one buddy if > size
        // for page_offset in unavailable_range {
        //     new.reserve(addr + page_offset * PAGE_SIZE, Order(0)).expect("exess memory reserved failed");
        // }
        Ok(new)
    }

    /// Returns the index of the buddy of order `order` starting at address `addr`.
    pub fn buddy_index(&self, addr: Page<T>, order: Order) -> usize {
        // debug_assert!(addr >= self.addr.into() && (addr - self.addr.into()) / PAGE_SIZE < self.max_order.nbr_pages().0);
        // debug_assert_eq!((addr - self.addr.into()) % order.nbr_bytes(), 0);

        self.first_layer_index(order) + (addr - self.addr).0 / (order.nbr_pages().0)
    }

    pub fn alloc(&mut self, order: Order) -> Result<Page<T>> {
        if order > self.max_order {
            return Err(MemoryError::OutOfBound);
        }

        match self.find_allocable_buddy(order) {
            Some(buddy_index) => {
                self.set_occupied(buddy_index, true);
                let layer_index = self.first_layer_index(order);
                debug_assert!(layer_index <= buddy_index);
                let buddy_layer_index: usize = buddy_index - layer_index;

                let addr = self.addr + order.nbr_pages() * buddy_layer_index;
                if addr < self.addr || (addr - self.addr) + order.nbr_pages() > self.size {
                    return Err(MemoryError::OutOfBound);
                }
                Ok(addr)
            }
            None => Err(MemoryError::OutOfMem),
        }
    }
    pub fn free(&mut self, addr: Page<T>, order: Order) -> Result<()> {
        let buddy_index = self.buddy_index(addr, order);

        if !self.get_buddy(buddy_index).occupied() || self.get_buddy(buddy_index).splitted() {
            Err(MemoryError::CannotFree)
        } else {
            self.set_occupied(buddy_index, false);
            Ok(())
        }
    }

    /// Reserves a buddy of order `order` starting at address `addr`.
    /// address reserved can be free the same way as an address returned by alloc
    /// # Panic
    /// panic if addr is not a multiple of order.nbr_pages() * PAGE_SIZE
    fn reserve(&mut self, addr: Page<T>, order: Order) -> Result<()> {
        if order > self.max_order
            || addr < self.addr
            || (addr - self.addr) + order.nbr_pages() > self.size
        {
            return Err(MemoryError::OutOfBound);
        }
        // print!("{:?},", addr.number);
        assert_eq!((addr - self.addr).0 % (order.nbr_pages()).0, 0);

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

    pub fn reserve_exact(&mut self, page: Page<T>, nbr_pages: NbrPages) -> Result<()> {
        // TODO: check if addr - self.addr = 0
        // TODO: handle errors
        for p in (page..page + nbr_pages).iter() {
            self.reserve(p, Order(0))?;
        }
        // loop {
        //     let starting_order = Order::sub_order(addr - self.addr);
        //     let reserve_order = Order::sub_order(nbr_pages);
        //     println!("{:x?},", nbr_pages);
        //     println!("{:?},", starting_order);
        //     println!("{:?},", reserve_order);
        //     if reserve_order <= starting_order {
        //         self.reserve(addr, reserve_order)?;
        //         if nbr_pages == reserve_order.nbr_pages() {
        //             break;
        //         }
        //         nbr_pages -= reserve_order.nbr_pages();
        //         addr += reserve_order.nbr_pages();
        //     } else {
        //         self.reserve(addr, starting_order)?;
        //         if nbr_pages == starting_order.nbr_pages() {
        //             break;
        //         }
        //         nbr_pages -= starting_order.nbr_pages();
        //         addr += starting_order.nbr_pages();
        //     }
        Ok(())
    }

    pub fn free_reserve(&mut self, addr: Page<T>, nbr_pages: NbrPages) -> Result<()> {
        for p in (addr..addr + nbr_pages).iter() {
            self.free(p, Order(0))?;
        }
        // loop {
        //     let starting_order = Order::sub_order(addr - self.addr);
        //     let reserve_order = Order::sub_order(nbr_pages);
        //     if reserve_order <= starting_order {
        //         self.free(addr, reserve_order)?;
        //     } else {
        //         self.free(addr, starting_order)?;
        //     }
        //     if nbr_pages <= starting_order.nbr_pages() {
        //         break;
        //     }
        //     nbr_pages -= starting_order.nbr_pages();
        //     addr += starting_order.nbr_pages();
        // }
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

    fn get_buddy(&mut self, index: usize) -> Buddy<'_> {
        Buddy::new(self.buddies.index_mut(index >> 2), (index & 0b11) as u8)
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

    fn find_allocable_buddy_aux(
        &mut self,
        target_depth: usize,
        current_depth: usize,
        index: usize,
    ) -> Option<usize> {
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

            self.find_allocable_buddy_aux(target_depth, current_depth + 1, left_index)
                .or_else(|| {
                    self.find_allocable_buddy_aux(target_depth, current_depth + 1, right_index)
                })
        } else {
            self.split_buddy(index);
            self.find_allocable_buddy_aux(
                target_depth,
                current_depth + 1,
                Self::left_child_index(index),
            )
        }
    }

    fn find_allocable_buddy(&mut self, order: Order) -> Option<usize> {
        self.find_allocable_buddy_aux(self.depth_buddy_from_order(order), 0, 0)
    }

    /// set occupied to `value` the buddy at index buddy and go back up the tree conserving the invariants
    fn set_occupied(&mut self, mut index: usize, value: bool) {
        self.get_buddy(index).set_occupied(value);
        if value == false {
            self.get_buddy(index).set_splitted(false);
        }

        while let Some(parent_index) = Self::parent_index(index) {
            let left_child = Self::left_child_index(parent_index);
            let right_child = Self::right_child_index(parent_index);

            if self.get_buddy(right_child).occupied() == value
                && self.get_buddy(left_child).occupied() == value
            {
                self.get_buddy(parent_index).set_occupied(value);
                if value == false
                    && self.get_buddy(right_child).splitted() == false
                    && self.get_buddy(left_child).splitted() == false
                {
                    self.get_buddy(parent_index).set_splitted(false);
                }
            } else if value == false {
                self.get_buddy(parent_index).set_occupied(false);
            }

            // if buddy is orphelin, set parent splitted recusivly
            if value == true {
                self.get_buddy(parent_index).set_splitted(true);
            }

            index = parent_index;
        }
    }

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

    pub fn metadata_size<O: Into<Order>>(order: O) -> usize {
        let order: Order = order.into();
        (Self::nbr_buddies(order.0) + 1) / 4
    }

    /// Returns the number of buddies of a BuddyAllocator of depth depth
    pub fn nbr_buddies(depth: usize) -> usize {
        2_usize.pow(depth as u32) * 2 - 1
    }

    pub fn ksize(&mut self, addr: Page<T>) -> Result<Order> {
        if addr < self.addr || addr - self.addr > self.size {
            return Err(MemoryError::OutOfBound);
        }

        let mut current_index = 0;
        let mut order = self.max_order;
        let target: usize = (addr - self.addr).to_bytes();
        let mut current_addr = 0;

        loop {
            if current_addr == target {
                let buddy = self.get_buddy(current_index);

                if buddy.occupied() && !buddy.splitted() {
                    break Ok(order);
                } else if !buddy.occupied() && !buddy.splitted() {
                    break Err(MemoryError::NotAllocated);
                } else {
                    current_index = Self::left_child_index(current_index);
                }
            } else {
                if target >= current_addr + (order - Order(1)).nbr_bytes() {
                    current_addr += (order - Order(1)).nbr_bytes();
                    current_index = Self::right_child_index(current_index);
                } else {
                    current_index = Self::left_child_index(current_index);
                }
            }
            order = order - Order(1);
        }
    }
}

pub struct Buddy<'a> {
    data: &'a mut u8,
    index: u8,
}

impl<'a> Debug for Buddy<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "[occupied: {}, splitted:{}]",
            self.occupied(),
            self.splitted()
        )
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

/// represent the order of a buddy:
/// order 0 <=> the smallest alloc <=> the liefes of the tree
/// order `max_order` <=> the greatest alloc <=> the root of the tree
#[derive(Debug, Copy, Clone, TryClone, PartialEq, Ord, PartialOrd, Eq)]
pub struct Order(pub usize);

impl Sub<Self> for Order {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(rhs <= self);
        Order(self.0 - rhs.0)
    }
}

impl Add<Self> for Order {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Order(self.0 + rhs.0)
    }
}

impl Order {
    #[inline(always)]
    pub fn nbr_pages(self) -> NbrPages {
        Into::<NbrPages>::into(self)
    }
    #[inline(always)]
    pub fn nbr_bytes(self) -> usize {
        Into::<usize>::into(self)
    }
}

impl From<Order> for NbrPages {
    #[inline(always)]
    fn from(order: Order) -> Self {
        NbrPages(1 << order.0)
    }
}

impl From<Order> for usize {
    #[inline(always)]
    fn from(order: Order) -> usize {
        Into::<NbrPages>::into(order).into()
    }
}

impl From<usize> for Order {
    #[inline(always)]
    fn from(nb_bytes: usize) -> Self {
        Into::<NbrPages>::into(nb_bytes).into()
    }
}

impl From<NbrPages> for Order {
    #[inline(always)]
    fn from(nbr_pages: NbrPages) -> Self {
        Order(nbr_pages.0.next_power_of_two().trailing_zeros() as usize)
    }
}

impl<T: Address> fmt::Display for BuddyAllocator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in &self.buddies {
            for i in 0..8 {
                write!(f, "{}", b.get_bit(i) as u8)?;
            }
            write!(f, " ")?;
        }
        write!(f, "\n")?;
        let mut offset = 0;
        for i in 0..=self.max_order.0 {
            for j in 0..(1 << i) {
                let index = offset + j;

                write!(
                    f,
                    "S:{:?}, ",
                    self.buddies[index >> 2].get_bit(((index & 0b11) << 1) as usize) as u8
                )?;
                write!(
                    f,
                    "O:{:?}  ",
                    self.buddies[index >> 2].get_bit(((index & 0b11) << 1) as usize + 1) as u8
                )?;
            }
            write!(f, "\n")?;
            offset += 1 << i;
        }
        Ok(())
    }
}

/// METADATA for a buddy wich address 2 ^ 20 pages (ie: 4GB)
/// multiply by 2 to get nb buddies
/// divide by 4 because a buddy is 2 bytes
const _METADATA_SIZE: usize = (1 << 20) * 2 / 4;

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::random::srand;
    use crate::math::random::srand_init;
    use crate::memory::tools::Virt;
    use core::ffi::c_void;
    #[test]
    fn sodo_allocator() {
        use std::alloc::{Alloc, Global, Layout, System};

        const NB_ALLOC: usize = 50000 / 10;
        let mut allocator: System = System;

        const NB_BLOCK: usize = 0x10000;
        let address_space = unsafe {
            allocator
                .alloc(Layout::from_size_align(NB_BLOCK * PAGE_SIZE, PAGE_SIZE).unwrap())
                .unwrap()
        };
        const MAX_ORDER: usize = NB_BLOCK.trailing_zeros() as usize;

        let mut buddy_allocator: BuddyAllocator<Virt> = BuddyAllocator::new(
            Virt(address_space.as_ptr() as usize).into(),
            NbrPages(NB_BLOCK),
        )
        .unwrap();

        // buddy_allocator.reserve_exact(Virt(address_space.as_ptr() as usize).into(), NbrPages(12));

        #[derive(Debug)]
        struct Allocation {
            order: Order,
            buddy_index: usize,
            random_u8: u8,
            ptr: Page<Virt>,
        }
        use core::fmt;
        use fmt::{Display, Formatter};

        impl Display for Allocation {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                let ptr: Virt = self.ptr.into();
                write!(
                    f,
                    "[{:x?}:{:x?}[, order: {}, random_byte: {:x}",
                    ptr,
                    ptr + self.order.nbr_bytes(),
                    self.order.0,
                    self.random_u8
                )
            }
        }

        srand_init(0xDEAD).unwrap();

        let mut allocations: Vec<Allocation> = vec![];

        for _nth_alloc in 0..NB_ALLOC {
            let type_alloc = srand::<u32>(3 - 1);
            match type_alloc {
                0 => {
                    let order = Order(srand::<usize>(MAX_ORDER / 2 - 1));
                    let nb_page = 1 << order.0;

                    //                        eprintln!("Attempting to allocate a region of order {} (nbr_pages: {})", order.0, order.nbr_pages());
                    let mem = buddy_allocator.alloc(order);
                    match mem {
                        Err(e) => eprintln!("Failed to allocate {:?}", e),
                        Ok(p) => {
                            let mem: Virt = p.into();
                            let mem = unsafe {
                                core::slice::from_raw_parts_mut(
                                    mem.0 as *mut u8,
                                    nb_page * PAGE_SIZE,
                                )
                            };
                            let random_u8 = srand(core::u8::MAX);
                            for c in mem.iter_mut() {
                                *c = random_u8;
                            }
                            let elem = Allocation {
                                order,
                                buddy_index: buddy_allocator.buddy_index(p, order),
                                ptr: p,
                                random_u8,
                            };
                            assert_eq!(elem.order, buddy_allocator.ksize(p).unwrap());
                            //                                eprintln!("Got {}\n", elem);
                            allocations.push(elem);
                        }
                    }
                }
                1 => {
                    if allocations.len() != 0 {
                        let index = srand::<usize>(allocations.len() - 1);
                        let elem = allocations.remove(index);
                        //                            eprintln!("Attempting to free {}", elem);
                        assert_eq!(
                            elem.buddy_index,
                            buddy_allocator.buddy_index(elem.ptr, elem.order)
                        );
                        assert_eq!(elem.order, buddy_allocator.ksize(elem.ptr).unwrap());
                        buddy_allocator
                            .free(elem.ptr, elem.order)
                            .expect("failed to free");
                        let ptr: Virt = elem.ptr.into();
                        let mem = unsafe {
                            core::slice::from_raw_parts_mut(ptr.0 as *mut u8, elem.order.into())
                        };
                        for (_i, c) in mem.iter().enumerate() {
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
                    }
                }
                2 => {
                    let order = Order(srand::<usize>(MAX_ORDER / 2 - 1));
                    let rand_max = (NB_BLOCK * PAGE_SIZE) / (order.nbr_pages().0 * PAGE_SIZE);
                    let addr = address_space.as_ptr() as usize
                        + srand::<usize>(rand_max - 1) * order.nbr_bytes();

                    let nb_page = 1 << order.0;

                    //                        eprintln!("Attempting to reserve a region [{:x}:{:x}[ of order {} (nbr_pages: {})", addr, addr + order.nbr_pages() * PAGE_SIZE, order.0, order.nbr_pages());
                    let p = Virt(addr).into();
                    let mem = buddy_allocator.reserve(p, order);
                    match mem {
                        Err(err) => eprintln!("Failed to reserve: {:?}", err),
                        Ok(_) => {
                            let mem = addr;
                            let mem = unsafe {
                                core::slice::from_raw_parts_mut(mem as *mut u8, nb_page * PAGE_SIZE)
                            };
                            let random_u8 = srand::<u8>(core::u8::MAX);
                            for c in mem.iter_mut() {
                                *c = random_u8;
                            }
                            let elem = Allocation {
                                order,
                                buddy_index: buddy_allocator.buddy_index(p, order),
                                ptr: p,
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
    fn test_free() {
        const NB_BLOCK: usize = 16;
        let map_location = 0x00010000 as *const u8;

        let mut buddy_allocator: BuddyAllocator<Virt> =
            BuddyAllocator::new(Virt(map_location as usize).into(), NbrPages(NB_BLOCK)).unwrap();

        let alloc_size = NbrPages(1);
        for i in 0..(NB_BLOCK) {
            let addr = buddy_allocator.alloc(alloc_size.into());
            dbg!(i);
            println!("{}", &buddy_allocator);
            assert_eq!(addr, Ok(Virt(map_location as usize + PAGE_SIZE * i).into()));
        }

        buddy_allocator
            .free(Virt(map_location as usize).into(), alloc_size.into())
            .expect("failed to free");
        for _ in 0..(NB_BLOCK) {
            let addr = buddy_allocator.alloc(alloc_size.into());
            println!("{}", &buddy_allocator);
            assert_eq!(addr, Ok(Virt(map_location as usize).into()));
            buddy_allocator
                .free(Virt(map_location as usize).into(), alloc_size.into())
                .expect("failed to free");

            println!("{}", &buddy_allocator);
        }
    }
    #[test]
    fn test_reserve_exact() {
        const NB_BLOCK: usize = 16;
        let map_location = 0x00010000 as *const u8;

        let mut buddy_allocator: BuddyAllocator<Virt> =
            BuddyAllocator::new(Virt(map_location as usize).into(), NbrPages(NB_BLOCK)).unwrap();
        buddy_allocator
            .reserve_exact(Virt(map_location as usize + PAGE_SIZE).into(), NbrPages(2))
            .unwrap();
        let buddy_before = buddy_allocator.clone();
        let alloc_size = NbrPages(1);
        for i in (0..NB_BLOCK).filter(|&i| !(i == 1 || i == 2)) {
            let addr = buddy_allocator.alloc(alloc_size.into());
            dbg!(i);
            assert_eq!(addr, Ok(Virt(map_location as usize + PAGE_SIZE * i).into()));
        }
        dbg!(&buddy_allocator);
        for i in (0..NB_BLOCK).filter(|&i| !(i == 1 || i == 2)) {
            dbg!(i);
            buddy_allocator
                .free(
                    Virt(map_location as usize + PAGE_SIZE * i).into(),
                    alloc_size.into(),
                )
                .expect("failed to free");
        }
        println!("{}", &buddy_before);
        println!("{}", &buddy_allocator);
        assert_eq!(buddy_before, buddy_allocator);
        buddy_allocator
            .free_reserve(Virt(map_location as usize + PAGE_SIZE).into(), NbrPages(2))
            .expect("failed to free");
    }
    #[test]
    fn sodo_buddy_fill() {
        use crate::math::random::rand;

        const NB_TESTS: usize = 10000 / 10;

        const NB_BLOCK: usize = 128;
        const PAGE_ORDER: usize = 4;

        let max_alloc = NB_BLOCK / PAGE_ORDER;
        let mut s: Vec<Virt> = vec![Virt(0); max_alloc];

        // First allocate a Buddy of NB_BLOCK
        let map_location = 0x100000 as *const u8;
        let mut buddy_allocator: BuddyAllocator<Virt> =
            BuddyAllocator::new(Virt(map_location as usize).into(), NbrPages(NB_BLOCK)).unwrap();

        let mut nb_allocations: usize = 0;

        // Multiple Sodo Alloc / Dealloc of same NbrPages
        for _i in 0..NB_TESTS {
            match rand::<bool>(true) {
                true => {
                    if max_alloc != nb_allocations {
                        let addr = buddy_allocator.alloc(NbrPages(PAGE_ORDER).into()).unwrap();
                        s[nb_allocations] = addr.into();
                        nb_allocations += 1;
                    }
                }
                false => match nb_allocations {
                    0 => {}
                    _ => {
                        let elmt_number = rand((nb_allocations - 1) as u32) as usize;
                        let virt_addr = s[elmt_number];
                        buddy_allocator
                            .free(virt_addr.into(), NbrPages(PAGE_ORDER).into())
                            .unwrap();
                        if elmt_number != nb_allocations - 1 {
                            s[elmt_number] = s[nb_allocations - 1];
                        }
                        nb_allocations -= 1;
                    }
                },
            }
        }

        // Fill the buddy until is full
        let empty_alloc = max_alloc - nb_allocations;
        for _i in 0..empty_alloc {
            buddy_allocator.alloc(NbrPages(PAGE_ORDER).into()).unwrap();
        }

        // Overflow the buddy, must return ERROR
        match buddy_allocator.alloc(NbrPages(PAGE_ORDER).into()) {
            Ok(_) => panic!("Must be overflowed"),
            Err(_e) => {}
        }
    }
}
