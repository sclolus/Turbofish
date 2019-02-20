use core::ffi::c_void;
use core::mem;
use core::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Buddy {
    inner: u8,
}

impl Buddy {
    fn left_child_index(i: usize) -> usize {
        i * 2 + 1
    }
    fn right_child_index(i: usize) -> usize {
        i * 2 + 2
    }

    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    gen_builder_pattern_bitfields_methods!(
        #[doc = " "],
        #[doc = " "],
        splitted, set_splitted, 1, inner);
    gen_builder_pattern_bitfields_methods!(
        #[doc = " "],
        #[doc = " "],
        occupied, set_occupied, 1, inner);
}

impl<'a> Index<usize> for Buddies<'a> {
    type Output = Buddy;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.index(index)
    }
}

// impl<'a> IndexMut<usize> for Buddies<'a> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.entries.index_mut(index)
//     }
// }

impl<'a> AsRef<[Buddy]> for Buddies<'a> {
    fn as_ref(&self) -> &[Buddy] {
        &self.entries
    }
}

// impl<'a> AsMut<[Buddy]> for Buddies<'a> {
//     fn as_mut(&mut self) -> &mut [Buddy] {
//         &mut self.entries
//     }
// }

pub struct Buddies<'a> {
    entries: &'a [Buddy],
}

static BUDDIES: Buddies = Buddies { entries: &[Buddy::new(); 0x400000] };

impl<'a> Buddies<'a> {
    fn left_child_index(i: usize) -> usize {
        i * 2 + 1
    }
    fn right_child_index(i: usize) -> usize {
        i * 2 + 2
    }
    /* fn alloc_aux(&mut self, pages_reclaim: usize, i: usize, depth: usize) -> Option<usize> {
        let curr = self[i];
        let curr_block_size = 1 << (20 - depth);
        if curr.occupied {
            return None;
        }
        if !curr.splitted
        alloc_aux(Buddies::left_child_index(i));

        alloc_aux(Buddies::right_child_index(i));
    }
    pub fn alloc(&mut self, pages_reclaim: usize) -> Option<usize> {
        //assert!(pages_reclaim.is_power_of_two());
        self.alloc_aux(pages_reclaim, 0, 0)

    }
    */
}

pub struct BuddyAllocator<'a> {
    addr: usize,
    size: usize,
    block_size: usize,
    max_order: u32,
    buddies: &'a mut [Buddy],
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(addr: usize, size: usize, block_size: usize, buddies: &'a mut [Buddy]) -> Self {
        assert!((size / block_size).is_power_of_two());
        assert!(addr % block_size == 0);

        let max_order = (size / block_size).trailing_zeros();
        println!("Max order: {} for buddy allocator at addr: {}", max_order, addr);

        let nbr_buddies = (2 * ((size / block_size) as usize)) - 1;
        println!("nbr_buddies = {}", nbr_buddies);

        //TODO: bzero memory for buddies
        for buddy in buddies.iter_mut() {
            buddy.set_occupied(false);
            buddy.set_splitted(false);
        }

        BuddyAllocator { addr, size, block_size, max_order, buddies }
    }

    fn split_buddy(&mut self, index: usize) -> Result<(), ()> {
        assert!(index < self.buddies.len() / 2 - 1);
        assert!(self.buddies[index].splitted() == false);
        assert!(self.buddies[index].occupied() == false);

        self.buddies[index].set_splitted(true);

        let left_index = Buddy::left_child_index(index);
        let right_index = Buddy::right_child_index(index);

        self.buddies[left_index].set_splitted(false);
        self.buddies[left_index].set_occupied(false);

        self.buddies[right_index].set_splitted(false);
        self.buddies[right_index].set_occupied(false);

        Ok(())
    }

    fn _find_allocable_buddy(&mut self, target_depth: usize, current_depth: usize, index: usize) -> Option<usize> {
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
            if let Some(buddy_index) =
                self._find_allocable_buddy(target_depth, current_depth + 1, Buddy::left_child_index(index))
            {
                return Some(buddy_index);
            }
            self._find_allocable_buddy(target_depth, current_depth + 1, Buddy::right_child_index(index))
        } else {
            if let Err(_) = self.split_buddy(index) {
                return None;
            }
            self._find_allocable_buddy(target_depth, current_depth + 1, Buddy::left_child_index(index))
        }
    }

    fn find_allocable_buddy(&mut self, target_depth: usize) -> Option<usize> {
        // while current_depth < target_depth {
        //     if !self.buddies[index].occupied() {
        //         if self.buddies[index].splitted() {
        //             let left_index = Buddy::left_child_index(index);
        //             let right_index = Buddy::right_child_index(index);

        //             if !self.buddies[left_index].occupied() {
        //                 index = left_index;
        //             } else if !self.buddies[right_index].occupied() {
        //                 index = right_index;
        //             } else {
        //                 return None;
        //             }

        //             current_depth += 1;
        //         } else {
        //             self.split_buddy(index).unwrap();
        //             return Some(Buddy::left_child_index(index));
        //         }
        //     } else if self.buddies[index].occupied() {
        //         return None;
        //     }
        // }
        self._find_allocable_buddy(target_depth, 0, 0)
    }

    pub fn buddy_addr(&self, index: usize) -> *const c_void {
        0x0 as *const c_void
    }

    pub fn alloc(&mut self, size: usize) -> Option<*const c_void> {
        let target_depth = {
            let mut buddy_size = 2usize.pow(self.max_order) * self.block_size;
            let mut target_depth = 0;

            while buddy_size / 2 >= size {
                // println!("$$$$$$$$$${:?} -> {:?} ({:?})", buddy_size, size, target_depth);
                buddy_size /= 2;
                target_depth += 1;
            }
            target_depth
        };
        // println!("Searching for target_depth: {}", target_depth);

        if let Some(buddy_index) = self.find_allocable_buddy(target_depth) {
            self.buddies[buddy_index].set_occupied(true);

            let depth_size = 2usize.pow((target_depth - 1) as u32);

            assert!(depth_size < buddy_index);
            let buddy_layer_index: usize = buddy_index - ((2 * depth_size) - 1);
            // println!(
            //     "Buddy size: {}, requested size: {}",
            //     (2usize.pow(self.max_order) * self.block_size) / 2usize.pow(target_depth as u32),
            //     size
            // );

            println!("Found buddy_index: {}", buddy_layer_index);
            let addr =
                2usize.pow(self.max_order - target_depth as u32) * self.block_size * buddy_layer_index + self.addr;

            Some(addr as *const c_void)
        } else {
            None
        }
    }
}
