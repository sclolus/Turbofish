use core::ffi::c_void;
use core::mem;
use core::ops::{Index, IndexMut};

use core::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Buddy {
    inner: u8,
}

impl Debug for Buddy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "[occupied: {}, splitted:{}]", self.occupied(), self.splitted())
    }
}

impl Buddy {
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    gen_builder_pattern_bitfields_methods!(
        #[doc = " "],
        #[doc = " "],
        splitted, set_splitted, 0, inner);
    gen_builder_pattern_bitfields_methods!(
        #[doc = " "],
        #[doc = " "],
        occupied, set_occupied, 1, inner);
}

pub struct BuddyAllocator<'a> {
    addr: usize,
    size: usize,
    block_size: usize,
    max_order: u32,
    pub buddies: &'a mut [Buddy],
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(addr: usize, size: usize, block_size: usize, buddies: &'a mut [Buddy]) -> Self {
        assert!((size / block_size).is_power_of_two());
        assert!(addr % block_size == 0);

        let max_order = (size / block_size).trailing_zeros();
        //println!("Max order: {} for buddy allocator at addr: {}", max_order, addr);

        let nbr_buddies = (2 * ((size / block_size) as usize)) - 1;
        //println!("nbr_buddies = {}", nbr_buddies);

        //TODO: bzero memory for buddies
        for buddy in buddies.iter_mut() {
            buddy.set_occupied(false);
            buddy.set_splitted(false);
        }

        BuddyAllocator { addr, size, block_size, max_order, buddies }
    }

    fn split_buddy(&mut self, index: usize) {
        assert!(index < self.buddies.len() / 2);
        assert!(self.buddies[index].splitted() == false);
        assert!(self.buddies[index].occupied() == false);

        self.buddies[index].set_splitted(true);

        let left_index = BuddyAllocator::left_child_index(index);
        let right_index = BuddyAllocator::right_child_index(index);

        self.buddies[left_index].set_splitted(false);
        self.buddies[left_index].set_occupied(false);

        self.buddies[right_index].set_splitted(false);
        self.buddies[right_index].set_occupied(false);
    }

    fn _find_allocable_buddy(&mut self, target_depth: usize, current_depth: usize, index: usize) -> Option<usize> {
        //dbg!(index);
        //dbg!(current_depth);
        if target_depth == current_depth {
            if self.buddies[index].occupied() {
                return None;
            }
            return Some(index);
        }

        if self.buddies[index].occupied() {
            return None;
        }

        if self.buddies[index].splitted() {
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

    pub fn buddy_addr(&self, index: usize) -> *const c_void {
        0x0 as *const c_void
    }

    /// size in bytes
    pub fn buddy_index(&self, addr: usize, size: usize) -> usize {
        assert!(addr % self.block_size == 0);
        assert!(addr >= self.addr && addr < self.addr + self.size);
        assert!(size >= self.block_size);
        assert!(size % self.block_size == 0);
        assert!(size <= self.block_size * 2usize.pow(self.max_order));
        let order = size.trailing_zeros() - self.block_size.trailing_zeros();

        (addr - self.addr) / (2usize.pow(order) * self.block_size) + (2usize.pow(self.max_order - order) - 1)
    }

    fn set_occupied(&mut self, mut index: usize, value: bool) {
        self.buddies[index].set_occupied(value);
        if value == false {
            self.buddies[index].set_splitted(false);
        }

        while let Some(parent_index) = BuddyAllocator::parent_index(index) {
            let left_child = BuddyAllocator::left_child_index(parent_index);
            let right_child = BuddyAllocator::right_child_index(parent_index);

            if self.buddies[right_child].occupied() == value && self.buddies[left_child].occupied() == value {
                self.buddies[parent_index].set_occupied(value);
                if value == false {
                    self.buddies[parent_index].set_splitted(false);
                }
            }

            index = parent_index;
        }
    }

    pub fn free(&mut self, addr: usize, size: usize) {
        self.set_occupied(self.buddy_index(addr, size), false)
    }

    fn depth_buddy_from_size(&self, size: usize) -> u32 {
        (self.max_order - (size.trailing_zeros() - self.block_size.trailing_zeros()))
    }

    fn first_layer_index(&self, depth: u32) -> usize {
        let sub_depth_size = 2usize.pow((depth - 1) as u32);
        2 * sub_depth_size - 1
    }

    /// size in bytes
    pub fn alloc(&mut self, size: usize) -> Option<*const c_void> {
        let target_depth = self.depth_buddy_from_size(size);
        //dbg!(target_depth);
        // println!("Searching for target_depth: {}", target_depth);

        if let Some(buddy_index) = self.find_allocable_buddy(target_depth as usize) {
            self.set_occupied(buddy_index, true);
            //dbg!(buddy_index);

            let layer_index = self.first_layer_index(target_depth);
            //dbg!(depth_size);

            assert!(layer_index <= buddy_index);
            let buddy_layer_index: usize = buddy_index - layer_index;
            //dbg!(buddy_layer_index);

            let addr =
                2usize.pow(self.max_order - target_depth as u32) * self.block_size * buddy_layer_index + self.addr;

            Some(addr as *const c_void)
        } else {
            None
        }
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
    static mut BUDDIES: [Buddy; ((1024 * 1024 * 1024) / 4096) * 2 - 1] =
        [Buddy::new(); (1024 * 1024 * 1024 / 4096) * 2 - 1];

    #[test]
    fn test_allocator() {
        let map_location = 0x00000000 as *const u8;
        let nb_block = 4;

        let mut buddy_allocator =
            unsafe { BuddyAllocator::new(map_location as usize, nb_block * 4096, 4096, &mut BUDDIES) };

        for i in 0..(nb_block) {
            let alloc_size = 4096;
            let mut addr = buddy_allocator.alloc(alloc_size);
            assert_eq!(addr, Some((alloc_size * i) as *const c_void));
        }
    }
}
