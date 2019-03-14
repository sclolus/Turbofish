extern crate libc;

mod nbr_pages;
use core::convert::{AsMut, AsRef};
use core::ffi::c_void;
use core::mem;
use nbr_pages::*;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
struct Node<T> {
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
    content: T,
}

impl<T> Node<T> {
    fn new(content: T) -> Self {
        Self {
            prev: None,
            next: None,
            content,
        }
    }

    fn prev(&self) -> Option<&Self> {
        unsafe { self.prev.map(|x| &*x) }
    }

    fn next(&self) -> Option<&Self> {
        unsafe { self.next.map(|x| &*x) }
    }

    fn prev_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.prev.map(|x| &mut *x) }
    }

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
        Self {
            head: None,
            tail: None,
            len: 0,
        }
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
        Iter {
            current: unsafe { self.head.map(|x| &*x) },
        }
    }

    pub fn iter_mut<'a>(&mut self) -> IterMut<'a, T> {
        IterMut {
            current: unsafe { self.head.map(|x| &mut *x) },
        }
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
        let next = self
            .current
            .as_mut()
            .and_then(|x| x.next.map(|x| unsafe { &mut *x }));
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
    elem_size: usize,
    elem_count: usize,
    // free_list: *mut LinkedList<,
    free_list: LinkedList<Vacant>,
    data: *mut u8,
}

impl core::ops::Drop for Slab {
    fn drop(&mut self) {
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
    const MAP_FAILED: usize = 0xffffffff;

    unsafe {
        match libc::mmap(core::ptr::null_mut(), size, 0b011, 0x20 | 0x2, -1, 0) as usize {
            MAP_FAILED => None,
            addr => Some(addr as *mut u8),
        }
    }
}

fn munmap(addr: *mut u8, size: usize) {
    unsafe {
        if libc::munmap(addr as *mut c_void, size) == -1 {
            panic!("Munmap failed for addr: {:p} size: {}", addr, size);
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
        for ptr in
            (1..self.elem_count).map(|x| (first_slot as usize + x * elem_size) as *mut FreeSlot)
        {
            self.free_list.push_back(ptr)
        }
    }

    pub fn new(mut elem_size: usize, elem_count: usize) -> Option<Self> {
        elem_size = elem_size.max(mem::size_of::<LinkedList<Vacant>>());
        let mut mmap_size = (elem_size * elem_count).max(PAGE_SIZE);
        // This clearly doesn't care if mmap_size is already a PAGE_SIZE multiple.
        mmap_size += PAGE_SIZE - mmap_size % PAGE_SIZE;

        let ptr = mmap(mmap_size)?;
        // dbg!(ptr);
        // dbg!(elem_size);
        // dbg!(elem_count);
        let mut new = Self {
            status: SlabStatus::Empty,
            nbr_pages: 1,
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

impl Cache {
    const BASE_NBR_SLABS: usize = 4;

    fn allocate_metadata(&mut self, nbr_slabs: usize) -> Result<(), ()> {
        assert!(nbr_slabs > self.nbr_slabs);
        let current_slabs_size = self.nbr_slabs * mem::size_of::<Node<Option<Slab>>>();
        let new_slabs_size = nbr_slabs * mem::size_of::<Node<Option<Slab>>>();

        let current_page_number = NbrPages::from(current_slabs_size).to_bytes();
        let new_page_number = NbrPages::from(new_slabs_size).to_bytes();

        dbg!(self.nbr_slabs);
        dbg!(nbr_slabs);

        let new = if current_page_number < new_page_number {
            mmap(new_page_number)
                .map(|addr| addr as *mut Node<Option<Slab>>)
                .ok_or(())?
        } else {
            self.data.unwrap()
        };

        let slice = unsafe { core::slice::from_raw_parts_mut(new, nbr_slabs) };
        let mut new_list = unsafe { LinkedList::in_place_construction(slice) };

        for new in new_list.iter_mut() {
            mem::forget(mem::replace(&mut new.content, None));
        }

        for (new, old) in new_list.iter_mut().zip(self.slabs.iter_mut()) {
            new.content = mem::replace(&mut old.content, None);
        }

        if let Some(data) = self.data {
            if data != new {
                println!("munmaping {:p}", data);
                munmap(data as *mut u8, NbrPages::from(self.nbr_slabs).into())
            }
        }

        self.nbr_slabs = nbr_slabs;
        self.data = Some(new);
        self.slabs = new_list;
        Ok(())
    }

    pub fn allocate_new_slab(&mut self) -> Option<&mut Slab> {
        match self.slabs.iter_mut().find(|node| node.content.is_none()) {
            // unsafe unwrap.
            Some(node) => {
                node.content = Some(Slab::new(self.elem_size, 64)?);
                Some(node.content.as_mut().unwrap())
            }
            None => {
                self.allocate_metadata(self.nbr_slabs * 2).ok()?;
                assert!(self.slabs.iter_mut().any(|x| x.content.is_none()));

                self.allocate_new_slab()
            }
        }
    }

    pub fn new(elem_size: usize) -> Self {
        let mut new = Cache {
            elem_size,
            nbr_slabs: 0,
            slabs: LinkedList::new(),
            data: None,
        };

        new.allocate_metadata(Self::BASE_NBR_SLABS).unwrap();
        new
    }

    pub fn alloc(&mut self) -> Option<*mut u8> {
        if let Some(slab) = self
            .slabs
            .iter_mut()
            .filter_map(|node| node.content.as_mut())
            .find(|slab| slab.status() != SlabStatus::Full)
        {
            slab.alloc()
        } else {
            self.allocate_new_slab()?;
            self.alloc()
        }
    }

    pub fn free(&mut self, addr: *mut u8) {
        if let Some(slab) = self
            .slabs
            .iter_mut()
            .filter_map(|node| node.content.as_mut())
            .find(|slab| slab.contains(addr))
        {
            slab.free(addr)
        } else {
            panic!("This cache does not contain this addr: {:p}", addr);
        }
    }
}

fn main() {
    const ALLOC_NBR: usize = 128;
    let mut slab = Slab::new(16, ALLOC_NBR).unwrap();
    let mut addrs: [Option<*mut u8>; ALLOC_NBR] = [None; ALLOC_NBR];

    for _ in 0..ALLOC_NBR {
        for (iteration, index) in (0..ALLOC_NBR).enumerate() {
            let addr = slab.alloc().unwrap();

            if let Some(addr) = addrs.iter().filter_map(|x| *x).find(|&x| x == addr) {
                panic!("{}: Addr: {:p} is already registered", iteration, addr);
            }

            addrs[index] = Some(addr);

            println!("{}: Allocated addr: {:p}", iteration, addr);
        }

        for (iteration, addr) in addrs.iter_mut().filter(|x| x.is_some()).enumerate() {
            println!("{}: Freeing {:p}", iteration, addr);
            slab.free((*addr).unwrap());
            *addr = None;
        }
        for free in slab.free_list.iter_mut() {
            println!("{:?}Yo dog", free);
        }
        for free in slab.free_list.iter() {
            println!("{:?}Yo", free);
        }

        break;
    }

    let mut cache = Cache::new(16);
    let mut addrs = Vec::new();

    for index in 0..32728 {
        let addr = cache.alloc().unwrap();
        if (index % 10 == 0) {
            println!("{}:{:?}", index, addr);
        }

        assert!(addrs.iter().all(|&x| x != addr));
        addrs.push(addr);
    }

    for (index, addr) in addrs.drain(..).enumerate() {
        println!("Freeing {} -> {:p}", index, addr);
        cache.free(addr);
    }
}
