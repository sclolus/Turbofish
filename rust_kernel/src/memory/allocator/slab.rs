use super::KERNEL_VIRTUAL_PAGE_ALLOCATOR;

use crate::memory::tools::*;
use core::alloc::Layout;
use core::convert::{AsMut, AsRef};
use core::fmt::Debug;
use core::mem;

#[derive(Debug)]
struct Node<T> {
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
    content: T,
}

impl<T> Node<T> {
    fn new(content: T) -> Self {
        Self { prev: None, next: None, content }
    }

    #[allow(dead_code)]
    fn prev(&self) -> Option<&Self> {
        unsafe { self.prev.map(|x| &*x) }
    }

    #[allow(dead_code)]
    fn next(&self) -> Option<&Self> {
        unsafe { self.next.map(|x| &*x) }
    }

    #[allow(dead_code)]
    fn prev_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.prev.map(|x| &mut *x) }
    }

    #[allow(dead_code)]
    fn next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.map(|x| &mut *x) }
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.content
    }
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
    len: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    fn push_front(&mut self, node: *mut Node<T>) {
        match self.head {
            Some(head) => unsafe {
                (*head).prev = Some(node);
                (*node).next = Some(head);
                (*node).prev = None;
                self.head = Some(node);
            },
            None => unsafe {
                self.head = Some(node);
                self.tail = Some(node);
                (*node).prev = None;
                (*node).next = None;
            },
        }
        self.len += 1;
    }

    fn push_back(&mut self, node: *mut Node<T>) {
        match self.tail {
            Some(tail) => unsafe {
                (*tail).next = Some(node);
                (*node).prev = Some(tail);
                (*node).next = None;
                self.tail = Some(node);
            },
            None => self.push_front(node),
        }
        self.len += 1;
    }

    fn remove_node(&mut self, node: *mut Node<T>) {
        unsafe {
            if let Some(next) = (*node).next {
                (*next).prev = (*node).prev;
            }
            if let Some(prev) = (*node).prev {
                (*prev).next = (*node).next;
            }

            if let Some(head) = self.head {
                if head == node {
                    self.head = (*node).next
                }
            }

            if let Some(tail) = self.tail {
                if tail == node {
                    self.tail = (*node).next
                }
            }
            (*node).next = None;
            (*node).prev = None;
        }
        self.len -= 1;
    }

    pub fn empty(&self) -> bool {
        if self.len == 0 {
            assert!(self.head.is_none());
            assert!(self.tail.is_none());
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter<'a>(&self) -> Iter<'a, T> {
        Iter { current: unsafe { self.head.map(|x| &*x) } }
    }

    pub fn iter_mut<'a>(&mut self) -> IterMut<'a, T> {
        IterMut { current: unsafe { self.head.map(|x| &mut *x) } }
    }

    pub unsafe fn in_place_construction(nodes: &mut [Node<T>]) -> Self {
        let mut new = Self::new();

        for node in nodes.iter_mut() {
            new.push_back(node as *mut Node<T>);
        }
        new
    }
}

struct Iter<'a, T: 'a> {
    current: Option<&'a Node<T>>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            self.current = current.next();
            Some(current)
        } else {
            None
        }
    }
}

struct IterMut<'a, T: 'a> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.as_mut().and_then(|x| x.next.map(|x| unsafe { &mut *x }));
        let current = self.current.take();
        self.current = next;

        current
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SlabStatus {
    Empty,
    Partial,
    Full,
}

/// ZST for the slab.
#[derive(Debug, Copy, Clone)]
struct Vacant;

#[derive(Debug)]
struct Slab {
    status: SlabStatus,
    nbr_pages: usize,
    pub elem_size: usize,
    elem_count: usize,
    free_list: LinkedList<Vacant>,
    data: *mut u8,
}

impl core::ops::Drop for Slab {
    fn drop(&mut self) {
        // println!("Dropping slab: {:p}", self);
        if self.free_list.len() != self.elem_count {
            println!(
                "Attempting to drop Slab: ({:p}, {}) while {} allocation are still active",
                self.data,
                self.elem_size,
                self.elem_count - self.free_list.len()
            );
            panic!()
            // println!("List of remaining active zones:");
            // Should add this later
        }
        munmap(self.data, self.nbr_pages * PAGE_SIZE);
    }
}

type FreeSlot = Node<Vacant>;
/// In bytes, probably should be a multiple of PAGE_SIZE
fn mmap(size: usize) -> Option<*mut u8> {
    unsafe {
        KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .alloc(size.into(), AllocFlags::KERNEL_MEMORY)
            .ok()
            .map(|addr| addr.to_addr().0 as *mut u8)
    }
}

fn munmap(addr: *mut u8, size: usize) {
    unsafe {
        if let Err(e) = KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().free(Page::containing(Virt(addr as usize))) {
            panic!("Failed to munmap {:p} size: {}: {:?}", addr, size, e);
        }
    }
}

impl Slab {
    unsafe fn init_free_list(&mut self) {
        self.free_list = LinkedList::new();

        let first_slot = self.data as *mut FreeSlot;
        *first_slot = Node::new(Vacant);
        self.free_list.push_front(first_slot);

        // This is how you satisfy the borrow checker. damn it.
        let elem_size = self.elem_size;
        for ptr in (1..self.elem_count).map(|x| (first_slot as usize + x * elem_size) as *mut FreeSlot) {
            self.free_list.push_back(ptr)
        }
    }

    pub fn new(mut elem_size: usize, elem_count: usize) -> Option<Self> {
        elem_size = elem_size.max(mem::size_of::<Node<Vacant>>());
        let mut mmap_size = (elem_size * elem_count).max(PAGE_SIZE);
        // This clearly doesn't care if mmap_size is already a PAGE_SIZE multiple.
        mmap_size += PAGE_SIZE - mmap_size % PAGE_SIZE;

        let ptr = mmap(mmap_size)?;
        // dbg!(ptr);
        // dbg!(elem_size);
        // dbg!(elem_count);
        let mut new = Self {
            status: SlabStatus::Empty,
            nbr_pages: NbrPages::from(mmap_size).0,
            elem_size,
            elem_count,
            free_list: LinkedList::new(),
            data: ptr as *mut u8,
        };
        unsafe {
            new.init_free_list();
        }

        Some(new)
    }

    fn update_status(&mut self) {
        self.status = match self.free_list.len() {
            0 => SlabStatus::Full,
            len if len != self.elem_count => SlabStatus::Partial,
            _ => SlabStatus::Empty,
        }
    }

    pub fn alloc(&mut self) -> Option<*mut u8> {
        // println!("Allocating inside slab: {:p}", self);
        if self.free_list.empty() {
            return None;
        }

        assert!(!self.free_list.empty());

        let current = self.free_list.head?;
        self.free_list.remove_node(current);
        self.update_status();
        Some(current as *mut u8)
    }

    pub fn free(&mut self, addr: *mut u8) {
        assert!(addr >= self.data);
        assert!(addr as usize <= self.data as usize + self.elem_count * self.elem_size);
        assert!((addr as usize - self.data as usize) % self.elem_size == 0);

        let vacant = addr as *mut FreeSlot;
        unsafe {
            *vacant = Node::new(Vacant);
        }
        self.free_list.push_front(vacant);
        self.update_status();
    }

    pub fn status(&self) -> SlabStatus {
        self.status
    }

    pub fn contains(&self, addr: *mut u8) -> bool {
        addr >= self.data
            && addr as usize <= self.data as usize + self.elem_count * self.elem_size
            && (addr as usize - self.data as usize) % self.elem_size == 0
    }
}

struct Cache {
    elem_size: usize,
    nbr_slabs: usize,
    slabs: LinkedList<Option<Slab>>,
    data: Option<*mut Node<Option<Slab>>>,
}

impl Debug for Cache {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "elem_size: {}\nnbr_slabs: {}\nslab: \n", self.elem_size, self.nbr_slabs)?;
        Ok(for (index, slab) in self.slabs.iter().enumerate() {
            writeln!(f, "slab {}:\n {:#?}", index, slab)?
        })
    }
}

impl Cache {
    const BASE_NBR_SLABS: usize = 4;

    fn allocate_metadata(&mut self, nbr_slabs: usize) -> core::result::Result<(), ()> {
        assert!(nbr_slabs > self.nbr_slabs);
        let current_slabs_size = self.nbr_slabs * mem::size_of::<Node<Option<Slab>>>();
        let new_slabs_size = nbr_slabs * mem::size_of::<Node<Option<Slab>>>();

        let current_page_number = NbrPages::from(current_slabs_size).to_bytes();
        let new_page_number = NbrPages::from(new_slabs_size).to_bytes();
        // println!(
        //     "Now allocating zone for {} slabs instead of {}, they span on {} pages\n",
        //     nbr_slabs, self.nbr_slabs, new_page_number
        // );

        // dbg!(self.nbr_slabs);
        // dbg!(nbr_slabs);

        let new = if current_page_number < new_page_number {
            mmap(new_page_number).map(|addr| addr as *mut Node<Option<Slab>>).ok_or(())?
        } else {
            self.data.ok_or(())?
        };
        let slice = unsafe { core::slice::from_raw_parts_mut(new, nbr_slabs) };
        let mut new_list = unsafe { LinkedList::in_place_construction(slice) };

        if let Some(addr) = self.data {
            if addr == new {
                let old_slice = unsafe { core::slice::from_raw_parts_mut(addr, nbr_slabs) };

                for new in &mut old_slice[self.nbr_slabs..nbr_slabs] {
                    mem::forget(mem::replace(&mut new.content, None));
                }
            } else {
                for new in &mut slice[self.nbr_slabs..nbr_slabs] {
                    mem::forget(mem::replace(&mut new.content, None));
                }
                for (new, old) in slice[0..self.nbr_slabs].iter_mut().zip(self.slabs.iter_mut()) {
                    mem::swap(&mut new.content, &mut old.content);
                    mem::forget(mem::replace(&mut old.content, None));
                }
            }
        } else {
            for new in new_list.iter_mut() {
                mem::forget(mem::replace(&mut new.content, None));
            }
        }

        if let Some(data) = self.data {
            if data != new {
                munmap(data as *mut u8, NbrPages::from(self.nbr_slabs).into())
            }
        }

        self.nbr_slabs = nbr_slabs;
        self.data = Some(new);
        self.slabs = new_list;
        // println!("Slabs allocation finished");
        Ok(())
    }

    pub fn allocate_new_slab(&mut self) -> core::result::Result<&mut Slab, ()> {
        match self.slabs.iter_mut().find(|node| node.content.is_none()) {
            Some(node) => {
                node.content = Some(Slab::new(self.elem_size, 128).ok_or(())?);
                Ok(node.content.as_mut().ok_or(())?)
            }
            None => {
                self.allocate_metadata(self.nbr_slabs * 2).ok().ok_or(())?;
                assert!(self.slabs.iter_mut().any(|x| x.content.is_none()));
                self.allocate_new_slab()
            }
        }
    }

    pub fn free_slab(&mut self, node: &mut Node<Option<Slab>>) {
        assert!(node.content.is_some());
        node.content = None;
    }

    pub fn new(elem_size: usize) -> core::result::Result<Self, ()> {
        let mut new = Cache { elem_size, nbr_slabs: 0, slabs: LinkedList::new(), data: None };

        new.allocate_metadata(Self::BASE_NBR_SLABS)?;
        Ok(new)
    }

    pub fn take_slab<F: Fn(&Slab) -> bool>(&mut self, predicate: F) -> Option<*mut Node<Option<Slab>>> {
        let found = self
            .slabs
            .iter_mut()
            .filter(|node| node.content.is_some())
            .find(|node| predicate(node.content.as_ref().expect("Can't fail")));

        found.map(|node| {
            let node_addr = node as *mut Node<Option<Slab>>;

            self.slabs.remove_node(node_addr);
            node_addr
        })
    }

    pub fn alloc(&mut self) -> Option<*mut u8> {
        let node = self.take_slab(|slab| slab.status != SlabStatus::Full);

        if let Some(node) = node {
            let slab = unsafe { (*node).content.as_mut().expect("Can't fail") }; // Can't fail.
            let addr = slab.alloc();
            let node = node as *mut Node<Option<Slab>>;

            if slab.status != SlabStatus::Full {
                self.slabs.push_front(node);
            } else {
                self.slabs.push_back(node);
            }
            addr
        } else {
            match self.allocate_new_slab() {
                Ok(_) => self.alloc(),
                Err(_) => None,
            }
        }

        // This code is more elegant but slower, however, this is not the bottleneck.
        // if let Some(slab) = self
        //     .slabs
        //     .iter_mut()
        //     .filter_map(|node| node.content.as_mut())
        //     .find(|slab| slab.status() != SlabStatus::Full)
        // {
        //     slab.alloc()
        // } else {
        //     self.allocate_new_slab()?;
        //     self.alloc()
        // }
    }

    pub fn free(&mut self, addr: *mut u8) {
        if let Some((_index, node)) = self
            .slabs
            .iter_mut()
            .enumerate()
            .filter(|(_, node)| node.content.is_some())
            .find(|(_, node)| node.content.as_ref().expect("Can't fail").contains(addr))
        {
            let slab = node.content.as_mut().expect("Can't fail");

            slab.free(addr);
            if slab.status() == SlabStatus::Empty {
                self.free_slab(node);
            }
        } else {
            panic!("This cache does not contain this addr: {:p}", addr);
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, addr: *mut u8) -> bool {
        self.slabs.iter().filter_map(|node| node.content.as_ref()).any(|slab| slab.contains(addr))
    }
}

impl core::ops::Drop for Cache {
    fn drop(&mut self) {
        // assert!(self.slabs.iter().all(|node| node.content.is_none()));
        let mut current_slabs_size = self.nbr_slabs * mem::size_of::<Node<Option<Slab>>>();
        current_slabs_size = NbrPages::from(current_slabs_size).to_bytes();

        if let Some(data) = self.data {
            munmap(data as *mut u8, current_slabs_size);
        }
    }
}

#[derive(Debug)]
pub struct SlabAllocator {
    caches: [Cache; 8],
}

impl SlabAllocator {
    pub fn new() -> Self {
        let caches = [
            Cache::new(32).expect("Failed to allocate Slab cache for length of 32"),
            Cache::new(64).expect("Failed to allocate Slab cache for length of 64"),
            Cache::new(128).expect("Failed to allocate Slab cache for length of 128"),
            Cache::new(256).expect("Failed to allocate Slab cache for length of 256"),
            Cache::new(512).expect("Failed to allocate Slab cache for length of 512"),
            Cache::new(1024).expect("Failed to allocate Slab cache for length of 1024"),
            Cache::new(2048).expect("Failed to allocate Slab cache for length of 2048"),
            Cache::new(4096).expect("Failed to allocate Slab cache for length of 4096"),
            // Cache::new(1 << 13),
            // Cache::new(1 << 14),
            // Cache::new(1 << 15),
            // Cache::new(1 << 16),
            // Cache::new(1 << 17),
            // Cache::new(1 << 18),
            // Cache::new(1 << 19),
            // Cache::new(1 << 20),
            // Cache::new(1 << 21),
            // Cache::new(1 << 22),
        ];
        // let mut caches = unsafe { mem::uninitialized() };
        // let mut base_size = 32;
        // for cache in caches.iter_mut() {
        //     mem::forget(mem::replace(cache, Cache::new(base_size)));
        //     base_size +=
        // }

        Self { caches }
    }

    pub fn alloc(&mut self, layout: Layout) -> Option<Virt> {
        let mut size = layout.size();
        if size < 32 {
            size = 32;
        }

        self.caches[size.next_power_of_two().trailing_zeros() as usize - 5 as usize]
            .alloc()
            .map(|addr| Virt(addr as usize))
    }

    pub fn free(&mut self, addr: Virt) -> Result<()> {
        let ptr = addr.0 as *mut u8;
        if let Some(cache) = self.caches.iter_mut().find(|cache| cache.contains(ptr)) {
            cache.free(ptr);
            Ok(())
        } else {
            // panic!("Tried to free non-allocated object: {:p}", addr);
            Err(MemoryError::NotAllocated)
        }
    }

    pub fn free_with_size(&mut self, addr: Virt, mut size: usize) {
        if size < 32 {
            size = 32;
        }
        let addr = addr.0 as *mut u8;
        let cache = &mut self.caches[size.next_power_of_two().trailing_zeros() as usize - 5 as usize];
        cache.free(addr);
    }

    pub fn ksize(&self, addr: Virt) -> Result<usize> {
        let ptr = addr.0 as *mut u8;
        if let Some(pos) = self.caches.iter().position(|cache| cache.contains(ptr)) {
            Ok(self.caches[pos].elem_size)
        } else {
            return Err(MemoryError::NotAllocated);
        }
    }
}

#[cfg(test)]
mod tests {
    // fn test_sodo() {
    //     use crate::math::random::srand;
    //     use crate::memory::tools::Virt;
    //     use core::alloc::Layout;
    //     crate::math::random::srand_init(42).unwrap();

    //     use super::SlabAllocator;
    //     let mut slab_allocator = SlabAllocator::new();
    //     const ALLOC_SIZE: usize = 2 << 17;
    //     let mut addrs: Vec<(*mut u8, u8, usize)> = Vec::with_capacity(32728);

    //     for _index in 0..32728 * 32 {
    //         // match rng.gen::<u8>() {
    //         match srand::<u8>(255) {
    //             0...200 => {
    //                 let alloc_size = srand(core::usize::MAX) % ALLOC_SIZE;
    //                 let addr = slab_allocator.alloc(Layout::from_size_align(alloc_size, 16).unwrap()).unwrap();
    //                 let object: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(addr.0 as *mut u8, alloc_size) };
    //                 let random_byte = srand::<u8>(255);

    //                 // for b in object.iter_mut() {
    //                 //     *b = random_byte;
    //                 // }

    //                 // assert!(addrs.iter().all(|&(x, _, _)| x != addr));
    //                 //                println!("Allocated object of size {} at: {:p}, filled with: {:x}", alloc_size, addr, random_byte);
    //                 addrs.push((addr.0 as *mut u8, random_byte, alloc_size));
    //             }
    //             _ => {
    //                 if addrs.len() == 0 {
    //                     continue;
    //                 }
    //                 let (addr, byte, alloc_size) = addrs.swap_remove(srand(core::usize::MAX) % addrs.len());

    //                 //                println!("Freeing {} -> {:p} filled with byte: {:x}", index, addr, byte);
    //                 let object: &[u8] = unsafe { core::slice::from_raw_parts(addr, alloc_size) };

    //                 // for b in object.iter() {
    //                 //     if *b != byte {
    //                 //         //                        println!("Failed to free {:p}, byte at {:p} is {:x} instead of {:x}", addr, b, *b, byte);
    //                 //         //                        println!("Zones which match incorrect byte: {}", byte);
    //                 //         for (_addr, _, _size) in addrs.iter().filter(|(_, pbyte, _)| byte == *pbyte) {
    //                 //             //                            println!("\t{:p} of size: {}", addr, size);
    //                 //         }
    //                 //     }
    //                 //     assert!(*b == byte);
    //                 // }
    //                 slab_allocator.free(Virt(addr as usize).into(), alloc_size);
    //                 //                println!("Free'd {:p}", addr);
    //             }
    //         }
    //     }
    //     for (addr, byte, alloc_size) in addrs.drain(..) {
    //         //        println!("Freeing -> {:p} filled with byte: {:x}", addr, byte);
    //         let object: &[u8] = unsafe { core::slice::from_raw_parts(addr, alloc_size) };

    //         // for b in object.iter() {
    //         //     if *b != byte {
    //         //         //                println!("Failed to free {:p}, byte at {:p} is {:x} instead of {:x}", addr, b, *b, byte);
    //         //     }
    //         //     assert!(*b == byte);
    //         // }

    //         //        println!("freeing: {:p}", addr);
    //         slab_allocator.free(Virt(addr as usize).into(), alloc_size);
    //         //        println!("free'd: {:p}", addr);
    //     }
    // }
}
