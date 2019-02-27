//! This is a dummy allocator.
use super::MemoryError;
use super::PAGE_SIZE;
use core::fmt::Debug;
use core::ops::Range;

type ResultAlloc<T> = Result<T, MemoryError>;

#[derive(Debug, Copy, Clone)]
struct LinkedList<T: Copy + Clone + Debug> {
    prev: Option<*mut LinkedList<T>>,
    next: Option<*mut LinkedList<T>>,
    content: T,
}

#[derive(Debug, Copy, Clone)]
struct LinkedListIter<'a, T>
where
    T: Copy + Clone + Debug,
{
    current: Option<&'a LinkedList<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T>
where
    T: Copy + Clone + Debug,
{
    type Item = &'a LinkedList<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        if let Some(current) = self.current {
            if current.next.is_some() {
                self.current = unsafe { Some(&*current.next.unwrap()) };
            } else {
                self.current = None
            }
        }
        current
    }
}

impl<T> LinkedList<T>
where
    T: Copy + Clone + Debug,
{
    fn new(content: T) -> Self {
        Self { prev: None, next: None, content }
    }

    fn push_back(&mut self, node: *mut Self) -> &mut Self {
        unsafe {
            let mut current = self as *mut Self;

            while let Some(next) = (*current).next {
                current = next;
            }

            (*node).prev = Some(current);

            (*current).next = Some(node);
            self
        }
    }

    #[allow(dead_code)]
    fn push_front(&mut self, node: *mut Self) -> &mut Self {
        unsafe {
            (*node).next = Some(self as *mut Self);

            self.prev = Some(node);
            &mut *node
        }
    }

    fn remove_node(mut self) -> Self {
        unsafe {
            if let Some(prev) = self.prev.take() {
                (*prev).next = self.next;
            }

            if let Some(next) = self.next.take() {
                (*next).prev = self.prev;
            }
            self
        }
    }

    fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter { current: Some(self) }
    }
}

#[derive(Debug, Copy, Clone)]
struct PageZone {
    ptr: *const u8,

    /// In number of pages.
    size: usize,
}

impl PageZone {
    fn new(ptr: *const u8, size: usize) -> Self {
        Self { ptr, size }
    }
}

impl PageZone {
    #[allow(dead_code)]
    fn contains(&self, range: Range<usize>) -> bool {
        let zone_range = self.ptr as usize..self.ptr as usize + PAGE_SIZE * self.size;

        zone_range.contains(&range.start) && zone_range.contains(&(range.end - 1))
    }

    fn overlaps(&self, range: Range<usize>) -> bool {
        let zone_range = self.ptr as usize..self.ptr as usize + PAGE_SIZE * self.size;

        zone_range.contains(&range.start)
            || zone_range.contains(&(range.end - 1))
            || range.contains(&zone_range.start)
            || range.contains(&(zone_range.end - 1))
    }
}

#[derive(Debug)]
pub struct DummyAllocator<'a> {
    // bitmap_pages: &mut [u8],
    addr: usize,
    page_size: usize,
    nbr_pages: usize,
    metadata: &'a mut [Option<LinkedList<PageZone>>],
    nbr_zones: usize,
    zones: Option<LinkedList<PageZone>>,
}

impl<'a> DummyAllocator<'a> {
    pub fn new(addr: usize, nbr_pages: usize, page_size: usize, metadata: &'a mut [u8]) -> Self {
        let metadata_addr = metadata as *mut [u8] as *mut Option<LinkedList<PageZone>>;
        let metadata_size = metadata.len();
        assert!(dbg!(core::mem::size_of::<LinkedList<PageZone>>() * nbr_pages) <= dbg!(metadata_size));

        let metadata = unsafe { core::slice::from_raw_parts_mut(metadata_addr, nbr_pages) };

        for entry in metadata.iter_mut() {
            *entry = None;
        }

        Self { addr, nbr_pages, page_size, metadata, nbr_zones: 0, zones: None }
    }

    fn allocate_new_zone(&mut self, ptr: *const u8, nbr_pages: usize) -> ResultAlloc<()> {
        let slot = match self.metadata.iter_mut().find(|x| x.is_none()) {
            Some(slot) => slot,
            // None => return Err("Out of metadata to store allocated zone"),
            None => return Err(MemoryError::OutOfMem),
        };

        *slot = Some(LinkedList::new(PageZone::new(ptr, nbr_pages)));
        self.nbr_zones += 1;

        let new_zone = slot.as_mut().unwrap() as *mut _;

        match &mut self.zones {
            Some(zones) => {
                zones.push_back(new_zone);
            }
            None => {
                self.zones = Some(unsafe { *new_zone });
            }
        }
        Ok(())
    }

    pub fn alloc(&mut self, nbr_pages: usize) -> Option<usize> {
        // println!("Attempting to allocate {} pages", nbr_pages);

        if let Some(addr) =
            (self.addr..self.addr + self.page_size * self.nbr_pages).step_by(self.page_size).find(|addr| {
                let addr_range = *addr..*addr + nbr_pages * self.page_size;
                let res = match &self.zones {
                    Some(zones) => zones.iter().all(|LinkedList { prev: _, next: _, content: zone }| {
                        // println!(
                        //     "zone_range: [{:x}:{:x}[, [{:x}:{:x}[",
                        //     zone.ptr as usize,
                        //     zone.ptr as usize + zone.size * self.page_size,
                        //     addr_range.start,
                        //     addr_range.end
                        // );
                        !zone.overlaps(addr_range.clone())
                    }),
                    None => true,
                };
                res
            })
        {
            self.allocate_new_zone(addr as *const u8, nbr_pages).ok()?;
            Some(addr)
        } else {
            None
        }
    }

    pub fn free(&mut self, addr: usize, nbr_pages: usize) {
        // println!("Attempting to free addr: {:x} of nbr_pages: {}", addr, nbr_pages);

        let node = self
            .zones
            .as_ref()
            .expect("Tried to free while no zones are currently allocated")
            .iter()
            .find(|LinkedList { prev: _, next: _, content: zone }| zone.ptr as usize == addr)
            .expect("Tried to free non-allocated zone");

        assert!(node.content.size == nbr_pages);
        node.remove_node();
        let next = self.zones.unwrap().next;
        if 0 == self.zones.unwrap().content.size {
            let next = if let Some(next) = next {
                let index = (next as usize - self.metadata as *const _ as *const u8 as usize)
                    / core::mem::size_of::<LinkedList<PageZone>>();

                self.metadata[index]
            } else {
                None
            };
            self.zones = next;
        }
    }

    // turbofish
    // unix
    // rust
    // bullshit
    // oriented
    // functional
    // interface
    // safety
    // harmonic
    pub fn reserve(&mut self, addr: usize, nbr_pages: usize) -> ResultAlloc<()> {
        // println!("Attempting to reserve addr: {:x} for {} pages", addr, nbr_pages);
        let addr = (addr / self.page_size) * self.page_size;
        assert!(addr % self.page_size == 0);

        let addr_range = addr..addr + nbr_pages * self.page_size;

        if match &self.zones {
            Some(zones) => {
                zones.iter().any(|LinkedList { prev: _, next: _, content: zone }| zone.overlaps(addr_range.clone()))
            }
            None => false,
        } {
            return Err(MemoryError::AlreadyMapped);
        } else {
            self.allocate_new_zone(addr as *const u8, nbr_pages)?;
            Ok(())
        }
    }
}

// struct Allocator<T>
// where
//     T: From<usize>;

// impl<T> Allocator<T>
// where T: From<usize> {
//     fn alloc(nbr_pages: usize) -> Option<T> {
//         let addr = 0x0;

//         Some(addr.into())
//     }
// }
#[cfg(test)]
mod test {
    use super::*;
    use core::ffi::c_void;
    #[test]
    fn sodo_allocator() {
        use rand::prelude::*;
        use std::alloc::{Alloc, Global, Layout, System};

        const NB_ALLOC: usize = 10000;
        let mut allocator: System = System;

        const NB_BLOCK: usize = 0x1000;
        let address_space =
            unsafe { allocator.alloc(Layout::from_size_align(NB_BLOCK * PAGE_SIZE, PAGE_SIZE).unwrap()).unwrap() };
        const MAX_ORDER: u32 = NB_BLOCK.trailing_zeros();

        static mut METADATA: [u8; 1024 * 1024 * 16 * 2] = [0u8; 1024 * 1024 * 16 * 2];

        let mut dummy_allocator =
            unsafe { DummyAllocator::new(address_space.as_ptr() as usize, NB_BLOCK, PAGE_SIZE, &mut METADATA) };

        #[derive(Debug)]
        struct Allocation<'a> {
            nb_page: usize,
            random_u8: u8,
            ptr: &'a mut [u8],
        }

        let mut rng: StdRng = StdRng::seed_from_u64(4);

        let mut allocations: Vec<Allocation> = vec![];

        for _nth_alloc in 0..NB_ALLOC {
            let type_alloc = rng.gen::<u32>() % 2;
            match type_alloc {
                0 => {
                    let order = rng.gen::<u32>() % (MAX_ORDER / 2);
                    let nb_page = 1 << order;
                    let mem = dummy_allocator.alloc(nb_page);
                    // dbg!(order);
                    // dbg!(nb_page);
                    // dbg!(mem);
                    // let mem = unsafe {
                    //     Some(
                    //         allocator
                    //             .alloc(Layout::from_size_align(nb_page * PAGE_SIZE, PAGE_SIZE).unwrap())
                    //             .unwrap()
                    //             .as_ptr() as usize,
                    //     )
                    // };
                    match mem {
                        None =>
                        // panic!("Allocated failed")
                        {
                            ()
                        }
                        Some(mem) => {
                            //                            eprintln!("mem: {:x}", mem);
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
                        //                        println!("desaloc");
                        let index = rng.gen::<usize>() % allocations.len();
                        let elem = allocations.remove(index);
                        for (_i, c) in elem.ptr.iter().enumerate() {
                            if *c != elem.random_u8 {
                                // dbg!(index);
                                // dbg!(i);
                                // dbg!(nth_alloc);
                                //                                eprintln!("{:p}, nbr_pages {}", elem.ptr as *const _, elem.nb_page);
                                if *c != elem.random_u8 {
                                    for _matching in allocations.iter().filter(|x| x.random_u8 == *c) {
                                        // eprintln!(
                                        //     "Allocation at {:p} has {} random_byte",
                                        //     matching.ptr as *const _, *c
                                        // );
                                    }
                                    assert_eq!(*c, elem.random_u8);
                                }
                            }
                        }
                        dummy_allocator.free(elem.ptr.as_ptr() as usize, elem.nb_page);

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
}
