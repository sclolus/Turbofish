use bit_field::BitField;
use core::ffi::c_void;
use core::fmt::Debug;
use core::mem;
use core::ops::{Index, IndexMut};

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
    pub buddies: &'a mut [u8],
    nbr_buddies: usize,
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(addr: usize, mut size: usize, block_size: usize, buddies: &'a mut [u8]) -> Self {
        size = size.next_power_of_two(); //TODO: set the unavailable area to occupied that is : [size.next_power_of_two - size..]
        assert!((size / block_size).is_power_of_two());
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
        BuddyAllocator { addr, size, block_size, max_order, buddies, nbr_buddies }
    }

    pub fn get_buddy(&mut self, index: usize) -> Buddy<'_> {
        Buddy::new(self.buddies.index_mut(index >> 2), index as u8 & 0b11)
    }

    fn split_buddy(&mut self, index: usize) {
        dbg!(index);
        dbg!(self.nbr_buddies);
        assert!(index < self.nbr_buddies / 2);
        assert!(self.get_buddy(index).splitted() == false);
        assert!(self.get_buddy(index).occupied() == false);

        self.get_buddy(index).set_splitted(true);

        let left_index = BuddyAllocator::left_child_index(index);
        let right_index = BuddyAllocator::right_child_index(index);

        self.get_buddy(left_index).set_splitted(false);
        self.get_buddy(left_index).set_occupied(false);

        self.get_buddy(right_index).set_splitted(false);
        self.get_buddy(right_index).set_occupied(false);
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
        if depth == 0 {
            return 0;
        }
        let sub_depth_size = 2usize.pow((depth - 1) as u32);
        2 * sub_depth_size - 1
    }

    /// size in bytes
    pub fn alloc(&mut self, size: usize) -> Option<*const c_void> {
        if size > self.size {
            return None;
        }

        let target_depth = self.depth_buddy_from_size(size);
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
