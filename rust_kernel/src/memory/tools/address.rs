use super::NbrPages;
use super::PAGE_SIZE;
use bit_field::BitField;
use core::ops::{Add, Range, RangeInclusive, Sub};

pub trait Address:
    From<usize>
    + Into<usize>
    + Sub<Self, Output = usize>
    + Sub<usize, Output = Self>
    + Add<usize, Output = Self>
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
    fn align_on(self, size: usize) -> <Self as Add<usize>>::Output {
        debug_assert!(size.is_power_of_two());

        let addr: usize = self.into();

        self + (!self.is_aligned_on(size)) as usize * (size - (addr & (size - 1)))
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtualAddr(pub usize);

impl VirtualAddr {
    #[inline(always)]
    pub fn pd_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    #[inline(always)]
    pub fn pt_index(&self) -> usize {
        self.0.get_bits(12..22)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn offset(&self) -> usize {
        self.0.get_bits(0..12)
    }
}

impl Into<usize> for VirtualAddr {
    #[inline(always)]
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for VirtualAddr {
    #[inline(always)]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl From<Page<VirtualAddr>> for VirtualAddr {
    #[inline(always)]
    fn from(page: Page<VirtualAddr>) -> Self {
        Self(page.number * PAGE_SIZE)
    }
}

impl Sub<VirtualAddr> for VirtualAddr {
    type Output = usize;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<usize> for VirtualAddr {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) - rhs)
    }
}

impl Add<usize> for VirtualAddr {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) + rhs)
    }
}

impl Address for VirtualAddr {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysicalAddr(pub usize);

impl Into<usize> for PhysicalAddr {
    #[inline(always)]
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for PhysicalAddr {
    #[inline(always)]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl From<Page<PhysicalAddr>> for PhysicalAddr {
    #[inline(always)]
    fn from(page: Page<PhysicalAddr>) -> Self {
        Self(page.number * PAGE_SIZE)
    }
}

impl Sub<usize> for PhysicalAddr {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) - rhs)
    }
}

impl Sub<PhysicalAddr> for PhysicalAddr {
    type Output = usize;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<usize> for PhysicalAddr {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        From::<usize>::from(Into::<usize>::into(self) + rhs)
    }
}

impl Address for PhysicalAddr {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page<T: Address> {
    pub number: usize,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Address> Page<T> {
    #[inline(always)]
    pub fn new(number: usize) -> Self {
        Self { number, _phantom: core::marker::PhantomData }
    }

    #[inline(always)]
    pub fn containing(addr: T) -> Self {
        From::from(addr)
    }

    #[inline(always)]
    pub fn inclusive_range(from: Self, to: Self) -> PageIter<T> {
        PageIter { current: from, end: to }
    }

    #[inline(always)]
    pub fn exclusive_range(from: Self, to: Self) -> PageIter<T> {
        let end = Self::new(to.number - 1);
        PageIter { current: from, end }
    }
}

impl Page<VirtualAddr> {
    #[inline(always)]
    pub fn pd_index(&self) -> usize {
        self.number.get_bits(10..20)
    }

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

impl<T: Address> From<T> for Page<T> {
    #[inline(always)]
    fn from(addr: T) -> Self {
        Self::new(addr.into() / PAGE_SIZE)
    }
}

pub trait IntoPageIter {
    type PageType: Address;
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

#[derive(Debug, Copy, Clone)]
pub struct PageIter<T: Address> {
    pub current: Page<T>,
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
        let addr = PhysicalAddr(PAGE_SIZE);
        let page: Page<PhysicalAddr> = addr.into();
        assert_eq!(page, Page::new(1));
        let convert_addr: PhysicalAddr = page.into();
        assert_eq!(convert_addr, addr);
    }
    #[test]
    fn test_page() {
        let page: Page<PhysicalAddr> = Page::new(1);
        assert_eq!(page + NbrPages(1), Page::<PhysicalAddr>::new(2));
    }
    #[test]
    fn test_page_iter() {
        let page_iter: PageIter<PhysicalAddr> = (Page::new(0)..=Page::new(10)).iter();
        for (i, page_frame) in page_iter.enumerate() {
            assert_eq!(Page::new(i), page_frame);
        }
        let all_page: Vec<Page<PhysicalAddr>> = page_iter.collect();
        assert_eq!(all_page.len(), 11);
        let page_iter: PageIter<PhysicalAddr> = (Page::new(0)..Page::new(10)).iter();
        for (i, page_frame) in page_iter.enumerate() {
            assert_eq!(Page::new(i), page_frame);
        }
        let all_page: Vec<Page<PhysicalAddr>> = page_iter.collect();
        assert_eq!(all_page.len(), 10);
    }
    #[test]
    fn test_align() {
        for i in 0..1000 {
            dbg!(i);
            dbg!(PhysicalAddr(i).align_on(4));
            assert!(PhysicalAddr(i).align_on(4).is_aligned_on(4));
            assert!(PhysicalAddr(i).align_on(4) >= PhysicalAddr(i));
        }
        assert_eq!(PhysicalAddr(1).align_on(4), PhysicalAddr(4));
    }
}
