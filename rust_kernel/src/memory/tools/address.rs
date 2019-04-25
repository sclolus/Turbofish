//! contains the new types representing an address and a page frame
use super::NbrPages;
use super::PAGE_SIZE;
use bit_field::BitField;
use core::fmt::Debug;
use core::ops::{Add, AddAssign, Range, RangeInclusive, Sub, SubAssign};

/// trait address common to physical and virtual address
pub trait Address:
    From<usize>
    + From<Page<Self>>
    + Into<usize>
    + Sub<Self, Output = usize>
    + Sub<usize, Output = Self>
    + Add<usize, Output = Self>
    + AddAssign<usize>
    + SubAssign<usize>
    + Debug
    + Copy
    + Clone
    + Ord
    + Eq
{
    /// size must be a power of two
    #[inline(always)]
    fn is_aligned_on(&self, size: usize) -> bool {
        debug_assert!(size.is_power_of_two());

        let addr: usize = (*self).into();

        (addr & (size - 1)) == 0
    }
    /// Align the address on a multiple of size
    /// size must be a power of two
    #[inline(always)]
    fn align_next(self, size: usize) -> <Self as Add<usize>>::Output {
        debug_assert!(size.is_power_of_two());

        let addr: usize = self.into();

        self + (!self.is_aligned_on(size)) as usize * (size - (addr & (size - 1)))
    }

    /// Align the address on the prev multiple of size
    /// size must be a power of two
    #[inline(always)]
    fn align_prev(self, size: usize) -> <Self as Sub<usize>>::Output {
        debug_assert!(size.is_power_of_two());

        let addr: usize = self.into();

        self - (addr & size - 1)
    }
    /// offset on the page <=> self % 4096
    #[inline(always)]
    fn offset(&self) -> usize {
        Into::<usize>::into(*self).get_bits(0..12)
    }
}

/// New type representing a Virtual Adress
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Virt(pub usize);

impl Virt {
    /// index on the page directory
    #[inline(always)]
    pub fn pd_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    /// index on the page table
    #[inline(always)]
    pub fn pt_index(&self) -> usize {
        self.0.get_bits(12..22)
    }
}

impl Into<usize> for Virt {
    #[inline(always)]
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for Virt {
    #[inline(always)]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl<T> From<*mut T> for Virt {
    #[inline(always)]
    fn from(addr: *mut T) -> Self {
        Self(addr as usize)
    }
}

impl<T> From<*const T> for Virt {
    #[inline(always)]
    fn from(addr: *const T) -> Self {
        Self(addr as usize)
    }
}

impl From<Page<Virt>> for Virt {
    #[inline(always)]
    fn from(page: Page<Virt>) -> Self {
        Self(page.number * PAGE_SIZE)
    }
}

impl Sub<Virt> for Virt {
    type Output = usize;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<usize> for Virt {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) - rhs)
    }
}

impl Add<usize> for Virt {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) + rhs)
    }
}

impl AddAssign<usize> for Virt {
    fn add_assign(&mut self, other: usize) {
        *self = *self + other
    }
}

impl SubAssign<usize> for Virt {
    fn sub_assign(&mut self, other: usize) {
        *self = *self - other
    }
}

impl Address for Virt {}

/// New type representing a Physical Adress
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Phys(pub usize);

impl Into<usize> for Phys {
    #[inline(always)]
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for Phys {
    #[inline(always)]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl From<Page<Phys>> for Phys {
    #[inline(always)]
    fn from(page: Page<Phys>) -> Self {
        Self(page.number * PAGE_SIZE)
    }
}

impl Sub<usize> for Phys {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) - rhs)
    }
}

impl Sub<Phys> for Phys {
    type Output = usize;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<usize> for Phys {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) + rhs)
    }
}

impl AddAssign<usize> for Phys {
    fn add_assign(&mut self, other: usize) {
        *self = *self + other
    }
}

impl SubAssign<usize> for Phys {
    fn sub_assign(&mut self, other: usize) {
        *self = *self - other
    }
}

impl Address for Phys {}

/// represent a page frame, wich can be virtual or physical
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page<T: Address> {
    /// the page number
    pub number: usize,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Address> Page<T> {
    /// new page frame from the page number
    #[inline(always)]
    pub fn new(number: usize) -> Self {
        Self { number, _phantom: core::marker::PhantomData }
    }

    /// page wich contains the address `addr`
    #[inline(always)]
    pub fn containing(addr: T) -> Self {
        From::from(addr)
    }

    /// create an iterator of the inclusive range (from..=to)
    #[inline(always)]
    pub fn inclusive_range(from: Self, to: Self) -> PageIter<T> {
        PageIter { current: from, end: to }
    }

    /// create an iterator of the exclusive range (from..to)
    #[inline(always)]
    pub fn exclusive_range(from: Self, to: Self) -> PageIter<T> {
        let end = Self::new(to.number - 1);
        PageIter { current: from, end }
    }

    /// convert to the address
    #[inline(always)]
    pub fn to_addr(self) -> T {
        From::<Page<T>>::from(self)
    }
}

impl Page<Virt> {
    /// index on the page directory
    #[inline(always)]
    pub fn pd_index(&self) -> usize {
        self.number.get_bits(10..20)
    }

    /// index on the page table
    #[inline(always)]
    pub fn pt_index(&self) -> usize {
        self.number.get_bits(0..10)
    }
}

impl<T: Address> Add<NbrPages> for Page<T> {
    type Output = Self;
    #[inline(always)]
    fn add(mut self, rhs: NbrPages) -> Self::Output {
        self.number += rhs.0;
        self
    }
}

impl<T: Address> AddAssign<NbrPages> for Page<T> {
    #[inline(always)]
    fn add_assign(&mut self, other: NbrPages) {
        *self = *self + other
    }
}

impl<T: Address> Sub<Self> for Page<T> {
    type Output = NbrPages;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        NbrPages(self.number - rhs.number)
    }
}

impl<T: Address> From<T> for Page<T> {
    #[inline(always)]
    fn from(addr: T) -> Self {
        Self::new(addr.into() / PAGE_SIZE)
    }
}

/// trait to implement into page iter on range traits of the Std
pub trait IntoPageIter {
    /// associate type representing Virtual or Physical page
    type PageType: Address;
    /// return the page iterator
    fn iter(self) -> PageIter<Self::PageType>;
}

impl<T: Address> IntoPageIter for Range<Page<T>> {
    type PageType = T;
    #[inline(always)]
    fn iter(self) -> PageIter<Self::PageType> {
        Page::exclusive_range(self.start, self.end)
    }
}

impl<T: Address> IntoPageIter for RangeInclusive<Page<T>> {
    type PageType = T;
    #[inline(always)]
    fn iter(self) -> PageIter<Self::PageType> {
        Page::inclusive_range(*self.start(), *self.end())
    }
}

/// Page iterator
#[derive(Debug, Copy, Clone)]
pub struct PageIter<T: Address> {
    /// curent page
    pub current: Page<T>,
    /// end page inclusive
    pub end: Page<T>,
}

impl<T: Address> Iterator for PageIter<T> {
    type Item = Page<T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let page = self.current;
            self.current.number = self.current.number + 1;
            Some(page)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_conversion_address() {
        let addr = Phys(PAGE_SIZE);
        let page: Page<Phys> = addr.into();
        assert_eq!(page, Page::new(1));
        let convert_addr: Phys = page.into();
        assert_eq!(convert_addr, addr);
    }
    #[test]
    fn test_page() {
        let page: Page<Phys> = Page::new(1);
        assert_eq!(page + NbrPages(1), Page::<Phys>::new(2));
    }
    #[test]
    fn test_page_iter() {
        let page_iter: PageIter<Phys> = (Page::new(0)..=Page::new(10)).iter();
        for (i, page_frame) in page_iter.enumerate() {
            assert_eq!(Page::new(i), page_frame);
        }
        let all_page: Vec<Page<Phys>> = page_iter.collect();
        assert_eq!(all_page.len(), 11);
        let page_iter: PageIter<Phys> = (Page::new(0)..Page::new(10)).iter();
        for (i, page_frame) in page_iter.enumerate() {
            assert_eq!(Page::new(i), page_frame);
        }
        let all_page: Vec<Page<Phys>> = page_iter.collect();
        assert_eq!(all_page.len(), 10);
    }
    #[test]
    fn test_align() {
        for i in 0..1000 {
            dbg!(i);
            dbg!(Phys(i).align_next(4));
            assert!(Phys(i).align_next(4).is_aligned_on(4));
            assert!(Phys(i).align_next(4) >= Phys(i));
        }
        assert_eq!(Phys(1).align_next(4), Phys(4));
    }
}
