use core::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Buddy {
    inner: u8,
}

impl Buddy {
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

struct BuddyAllocator<'a> {
    addr: usize,
    size: usize,
    block_size: usize,
    max_order: u32,
    buddies: &'a mut [Buddy],
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(addr: usize, size: usize, block_size: usize) -> Self {
        assert!((addr / block_size).is_power_of_two());

        let max_order = (addr / block_size).trailing_zeros();

        BuddyAllocator {
            addr,
            size,
            block_size,
            max_order,
            buddies: unsafe {
                core::slice::from_raw_parts_mut(
                    addr as *mut _,
                    core::mem::size_of::<Buddy>() * ((2 * (max_order as usize)) - 1),
                )
            },
        }
    }
}
