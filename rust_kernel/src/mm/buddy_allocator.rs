use super::MemoryError;
use bit_field::BitField;
use core::fmt::Debug;
use core::ops::IndexMut;

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

    // gen_builder_pattern_bitfields_methods!(
    //     #[doc = " "],
    //     #[doc = " "],
    //     splitted, set_splitted, 0, inner);
    // gen_builder_pattern_bitfields_methods!(
    //     #[doc = " "],
    //     #[doc = " "],
    //     occupied, set_occupied, 1, inner);

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

pub struct BuddyAllocator<'a> {
    addr: usize,
    size: usize,
    block_size: usize,
    max_order: u32,
    /// Invariant: all unused buddies are zeroed
    pub buddies: &'a mut [u8],
    nbr_buddies: usize,
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(addr: usize, mut size: usize, block_size: usize, buddies: &'a mut [u8]) -> Self {
        let normalized_size = size.next_power_of_two();
        let unavailable_range = addr + size..addr + normalized_size;

        size = normalized_size; //TODO: set the unavailable area to occupied that is : [size.next_power_of_two - size..]... Now supposedly done.
        assert!((size / block_size).is_power_of_two());
        assert!(block_size.is_power_of_two());
        assert!(addr % block_size == 0);

        let max_order = (size / block_size).trailing_zeros();
        //println!("Max order: {} for buddy allocator at addr: {}", max_order, addr);

        let nbr_buddies = (2 * ((size / block_size) as usize)) - 1;
        assert!(buddies.len() * 4 >= nbr_buddies);
        //println!("nbr_buddies = {}", nbr_buddies);

        //TODO: bzero memory for buddies
        for byte in buddies.iter_mut() {
            *byte = 0;
        }

        let buddies_len = buddies.len();
        let mut new = BuddyAllocator { addr, size, block_size, max_order, buddies, nbr_buddies };
        let buddies_addr = new.buddies.as_mut_ptr() as usize;

        let range = addr..addr + size;
        if range.contains(&buddies_addr) {
            // You clearly are trying to bullshit me if you provided a partially overlapping buddies slice this function.
            assert!(range.contains(&(buddies_addr + buddies_len)));
            new.reserve(buddies_addr, (buddies_len) / new.block_size + (buddies_len % new.block_size != 0) as usize)
                .expect("self reserve buddy alocator failed");
        }
        for p in unavailable_range.step_by(new.block_size) {
            new.reserve(p, 1).expect("exess memory reserved failed");
        }
        new
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

    fn find_allocable_buddy(&mut self, target_depth: usize) -> Option<usize> {
        self._find_allocable_buddy(target_depth, 0, 0)
    }

    /// size in number of pages.
    pub fn buddy_index(&self, addr: usize, size: usize) -> usize {
        assert!(addr % self.block_size == 0);
        assert!(addr >= self.addr && addr < self.addr + self.size);
        assert!(size * self.block_size <= self.size);
        assert!(size.is_power_of_two());

        let order = size.trailing_zeros();

        (addr - self.addr) / (2usize.pow(order) * self.block_size) + (2usize.pow(self.max_order - order) - 1)
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

    pub fn free(&mut self, addr: usize, nbr_pages: usize) {
        self.set_occupied(self.buddy_index(addr, nbr_pages.next_power_of_two()), false)
    }

    /// size in number of pages.
    fn depth_buddy_from_size(&self, size: usize) -> u32 {
        self.max_order - size.trailing_zeros()
    }

    fn first_layer_index(&self, depth: u32) -> usize {
        if depth == 0 {
            return 0;
        }
        let sub_depth_size = 2usize.pow((depth - 1) as u32);
        2 * sub_depth_size - 1
    }

    /// size in bytes
    pub fn alloc(&mut self, nbr_pages: usize) -> Option<usize> {
        if nbr_pages.checked_mul(self.block_size)? > self.size {
            return None;
        }

        let target_depth = self.depth_buddy_from_size(nbr_pages);
        dbg!(target_depth);
        // println!("Searching for target_depth: {}", target_depth);

        if let Some(buddy_index) = self.find_allocable_buddy(target_depth as usize) {
            self.set_occupied(buddy_index, true);
            //dbg!(buddy_index);

            let layer_index = self.first_layer_index(target_depth);
            //dbg!(depth_size);

            dbg!(layer_index);
            dbg!(buddy_index);
            assert!(layer_index <= buddy_index);
            let buddy_layer_index: usize = buddy_index - layer_index;
            //dbg!(buddy_layer_index);

            let addr =
                2usize.pow(self.max_order - target_depth as u32) * self.block_size * buddy_layer_index + self.addr;

            dbg!(addr);

            Some(addr)
        } else {
            None
        }
    }
    /// Reserve atleast the `range` specified.
    pub fn reserve(&mut self, addr: usize, nb_page: usize) -> Result<(), MemoryError> {
        // TODO: optimize that bu segmeting
        let end = addr + nb_page * self.block_size;
        let normalized_start = addr - (addr & (self.block_size - 1));
        let normalized_end = if (end & (self.block_size - 1)) != 0 {
            end + (self.block_size - (end & (self.block_size - 1)))
        } else {
            end
        };
        let size = (normalized_end - normalized_start).next_power_of_two() / self.block_size;
        // assert!(size.is_power_of_two());
        dbg!(self.addr);
        dbg!(self.size);
        dbg!(normalized_start);
        dbg!(normalized_end);

        if normalized_end > self.addr + self.size || normalized_start < self.addr {
            return Err(MemoryError::OutOfBound);
        }

        let index = self.buddy_index(normalized_start, size);
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
        const max_order: u32 = nb_block.trailing_zeros();

        static mut BUDDIES: [u8; (nb_block * 2 - 1) / 4 + 1] = [0u8; (nb_block * 2 - 1) / 4 + 1];

        let mut buddy_allocator = unsafe {
            BuddyAllocator::new(address_space.as_ptr() as usize, nb_block * PAGE_SIZE, PAGE_SIZE, &mut BUDDIES)
        };

        #[derive(Debug)]
        struct Allocation<'a> {
            nb_page: usize,
            random_u8: u8,
            ptr: &'a mut [u8],
        }

        let mut rng: StdRng = StdRng::seed_from_u64(4);

        let mut allocations: Vec<Allocation> = vec![];

        for nth_alloc in 0..NB_ALLOC {
            let type_alloc = rng.gen::<u32>() % 2;
            match type_alloc {
                0 => {
                    let order = rng.gen::<u32>() % (max_order / 2);
                    let nb_page = 1 << order;
                    let mem = buddy_allocator.alloc(nb_page);
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
                            allocations.push(Allocation { nb_page, ptr: mem, random_u8 });
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
                        buddy_allocator.free(elem.ptr.as_ptr() as usize, elem.nb_page);

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
            unsafe { BuddyAllocator::new(map_location as usize, nb_block * 4096, 4096, &mut BUDDIES) };

        for i in 0..(nb_block) {
            let alloc_size = 1;
            let mut addr = buddy_allocator.alloc(1);
            assert_eq!(addr, Some(4096 * i));
        }
    }
}
