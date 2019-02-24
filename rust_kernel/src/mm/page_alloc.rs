use super::PAGE_DIRECTORY;
use super::PAGE_TABLES;
use bit_field::BitField;
use core::ffi::c_void;
use core::fmt::Debug;
use core::mem;
use core::ops::{Index, IndexMut, Range};

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
        let normalized_size = size.next_power_of_two();
        let unavailable_range = addr + size..addr + size + (normalized_size - size);

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
            new.reserve(buddies_addr..buddies_addr + buddies_len);
        }

        new.reserve(unavailable_range);
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

    fn buddy_addr(&self, index: usize) -> usize {
        0x0
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
            if value == true {
                self.get_buddy(parent_index).set_splitted(true);
            }

            index = parent_index;
        }
    }

    pub fn free(&mut self, addr: usize, nbr_pages: usize) {
        self.set_occupied(self.buddy_index(addr, nbr_pages), false)
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
    pub fn reserve(&mut self, range: Range<usize>) {
        if range.start == range.end {
            return;
        }

        let normalized_start = range.start - (range.start & (self.block_size - 1));
        let normalized_end = range.end + (self.block_size - (range.end & (self.block_size - 1)));
        let size = (normalized_end - normalized_start).next_power_of_two() / self.block_size;
        // assert!(size.is_power_of_two());

        let index = self.buddy_index(normalized_start, size);

        self.set_occupied(index, true);
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PhysicalAllocatorType {
    Normal,
    Dma,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum VirtualAllocatorType {
    KernelSpace,
    UserSpace,
}

const PAGE_CONST: u8 = 0x2;
const PAGE_CACHABLE_DISABLED: u8 = 0x4;
const PAGE_USER: u8 = 0x8;
const MAY_SLEEP: u8 = 0x10;
const NO_RETRY: u8 = 0x40;
const PAGE_DMA: u8 = 0x80;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct AllocFlags(u8);

const ALLOC_DMA: u8 = PAGE_CACHABLE_DISABLED | PAGE_DMA;
const ALLOC_ATOMIC: u8 = NO_RETRY;
const ALLOC_NORMAL: u8 = MAY_SLEEP;

struct PageAllocator<'a, 'b: 'a> {
    //different lifetimes for each type of buddy allocators ?
    zoned_physical_buddy_allocators: &'b mut [(BuddyAllocator<'a>, PhysicalAllocatorType)],
    zoned_virtual_buddy_allocators: &'b mut [(BuddyAllocator<'a>, VirtualAllocatorType)],
}

impl<'a, 'b: 'a> PageAllocator<'a, 'b> {
    // pub fn new() -> Self {
    //     //??

    // }

    pub fn alloc(&mut self, nbr_pages: usize, flags: AllocFlags) -> Option<usize> {
        use PhysicalAllocatorType::*;
        use VirtualAllocatorType::*;
        let pbuddy;
        let vbuddy;
        let requested_pbuddy_type;
        let requested_vbuddy_type;

        if flags.0 & PAGE_DMA != 0 {
            requested_pbuddy_type = Dma;
        } else {
            requested_pbuddy_type = Normal;
        }
        pbuddy = self
            .zoned_physical_buddy_allocators
            .iter_mut()
            .find(|(buddy, btype)| *btype == requested_pbuddy_type)
            .map(|(buddy, _)| buddy)?;

        if flags.0 & PAGE_USER != 0 {
            requested_vbuddy_type = UserSpace;
        } else {
            requested_vbuddy_type = KernelSpace;
        }
        vbuddy = self
            .zoned_virtual_buddy_allocators
            .iter_mut()
            .find(|(buddy, btype)| *btype == requested_vbuddy_type)
            .map(|(buddy, _)| buddy)?;

        let vaddr = vbuddy.alloc(nbr_pages)?;
        let paddr = pbuddy.alloc(nbr_pages).or_else(|| {
            vbuddy.free(vaddr, nbr_pages);
            None
        })?;

        unsafe {
            PAGE_DIRECTORY
                .remap_addr(vaddr, paddr)
                .or_else(|err| {
                    vbuddy.free(vaddr, nbr_pages);
                    pbuddy.free(vaddr, nbr_pages);
                    Err(err)
                })
                .ok()?; //OK SO THIS DOES NOT MAP ALL THE PAGES FIX THIS..
        }

        Some(vaddr)
    }

    pub fn free(&mut self, addr: usize, nbr_pages: usize) {}
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
