use super::SECTOR_SIZE;
use core::ops::{Add, Sub};

/// new type representing a number of sectors
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NbrSectors(pub u64);

impl Into<usize> for NbrSectors {
    fn into(self) -> usize {
        self.0 as usize * SECTOR_SIZE
    }
}

impl From<usize> for NbrSectors {
    fn from(u: usize) -> Self {
        Self((u / SECTOR_SIZE + if u % SECTOR_SIZE != 0 { 1 } else { 0 }) as u64)
    }
}

/// Add boilerplate for Sector + NbrSectors
impl Sub<NbrSectors> for NbrSectors {
    type Output = Self;

    fn sub(self, other: NbrSectors) -> Self::Output {
        Self(self.0 - other.0)
    }
}
/// new type representing the start sector
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Sector(pub u64);

/// Add boilerplate for Sector + NbrSectors
impl Add<NbrSectors> for Sector {
    type Output = Sector;

    fn add(self, other: NbrSectors) -> Self::Output {
        Self(self.0 + other.0)
    }
}

use crate::memory::mmu::Entry;
use crate::memory::{mmap, munmap};
use core::fmt;
use core::fmt::Debug;

pub struct MemoryMapped<T: Copy> {
    pub inner: *mut T,
}

impl<T: Copy + Debug> Debug for MemoryMapped<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

impl<T: Copy> MemoryMapped<T> {
    /// take a physical addr and mmap the addr in noncachable way
    pub fn new(p: *mut T) -> Result<Self> {
        unsafe { Ok(Self { inner: mmap(p, Entry::CACHE_DISABLE)? }) }
    }
    /// read volatile the underlying data
    pub fn get(&self) -> T {
        unsafe { core::ptr::read_volatile(self.inner) }
    }
    /// read volatile of a part of a part of inner data
    pub fn get_precise<U>(&self, offset: usize) -> U {
        unsafe { core::ptr::read_volatile((self.inner as *mut u8).add(offset) as *mut U) }
    }
    /// write volatile the underlying data
    pub fn set(&mut self, t: T) {
        unsafe { core::ptr::write_volatile(self.inner, t) }
    }
    /// write volatile just a part of underlying data
    pub fn set_precise<U>(&mut self, offset: usize, t: U) {
        unsafe { core::ptr::write_volatile((self.inner as *mut u8).add(offset) as *mut U, t) }
    }
    /// apply f on the underlying data
    pub fn update<F: FnOnce(T) -> T>(&mut self, f: F) -> T {
        let old = self.get();
        let new = f(old);
        self.set(new);
        new
    }
}

impl<T: Copy> Drop for MemoryMapped<T> {
    fn drop(&mut self) {
        unsafe {
            munmap(self.inner).unwrap();
        }
    }
}

use crate::memory::allocator::kernel::{kfree, kmalloc};
use crate::memory::tools::*;
use core::borrow::{Borrow, BorrowMut};

/// Basicly a Box in which one's can customize the allocation with AllocFlags
pub struct CustomBox<T> {
    /// the alloced ptr
    ptr: *mut T,
    /// remember the flags for clone
    flags: AllocFlags,
}

impl<T> CustomBox<T> {
    pub fn new(t: T, flags: AllocFlags) -> Self {
        dbg!(core::mem::size_of::<T>());
        dbg!(core::mem::align_of::<T>());
        let ptr: *mut T = kmalloc(flags);
        eprintln!("{:?}", ptr);
        unsafe { ptr.write(t) };
        Self { ptr, flags }
    }
    pub fn mut_ptr(&mut self) -> *mut T {
        self.as_mut() as *mut T
    }
    pub fn ptr(&self) -> *const T {
        self.as_ref() as *const T
    }
}

impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        kfree(self.ptr)
    }
}

impl<T: Clone> Clone for CustomBox<T> {
    fn clone(&self) -> Self {
        unsafe { Self::new((*self.ptr).clone(), self.flags) }
    }
}

impl<T> AsRef<T> for CustomBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> AsMut<T> for CustomBox<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Borrow<T> for CustomBox<T> {
    fn borrow(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> BorrowMut<T> for CustomBox<T> {
    fn borrow_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}
