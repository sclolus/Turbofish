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

impl Index<usize> for Buddies {
    type Output = Buddy;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.index(index)
    }
}

impl IndexMut<usize> for Buddies {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.index_mut(index)
    }
}

impl AsRef<[Buddy]> for Buddies {
    fn as_ref(&self) -> &[Buddy] {
        &self.entries
    }
}

impl AsMut<[Buddy]> for Buddies {
    fn as_mut(&mut self) -> &mut [Buddy] {
        &mut self.entries
    }
}

pub struct Buddies {
    entries: [Buddy; 0x400000],
}

static BUDDIES: Buddies = Buddies { entries: [Buddy::new(); 0x400000] };

impl Buddies {
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
