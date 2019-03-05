use super::PAGE_SIZE;
use bit_field::BitField;
use core::ops::{Add, Range, RangeInclusive};

pub trait Address: From<usize> + Into<usize> + Add<usize> + Copy + Clone + Ord + Eq {
    #[inline(always)]
    fn is_aligned_on(&self, size: usize) -> bool {
        let addr: usize = (*self).into();

        (addr & (size - 1)) == 0
    }
    #[inline(always)]
    /// Align the address on a multiple of size
    /// size must be a power of two
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
    #[inline]
    pub fn pd_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    #[inline]
    pub fn pt_index(&self) -> usize {
        self.0.get_bits(12..22)
    }

    #[inline]
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

impl Add<usize> for VirtualAddr {
    type Output = Self;
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

impl Add<usize> for PhysicalAddr {
    type Output = Self;
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

// impl<T: Address> Into<T> for Page<T> {
//     fn into(self) -> T {
//         let number: usize = self.number;
//         (number * PAGE_SIZE).into()
//     }
// }

impl<T: Address> From<T> for Page<T> {
    fn from(addr: T) -> Self {
        Self::new(addr.into() / PAGE_SIZE)
    }
}

impl<T: Address> Page<T> {
    pub fn new(number: usize) -> Self {
        Self { number, _phantom: core::marker::PhantomData }
    }

    pub fn inclusive_range(from: Self, to: Self) -> PageIter<T> {
        PageIter { current: from, end: to }
    }

    pub fn exclusive_range(from: Self, to: Self) -> PageIter<T> {
        let end = Self::new(to.number - 1);
        PageIter { current: from, end }
    }
}

pub trait IntoPageIter {
    type PageType: Address;
    fn iter(self) -> PageIter<Self::PageType>;
}

impl<T: Address> IntoPageIter for Range<Page<T>> {
    type PageType = T;
    fn iter(self) -> PageIter<Self::PageType> {
        Page::exclusive_range(self.start, self.end)
    }
}

impl<T: Address> IntoPageIter for RangeInclusive<Page<T>> {
    type PageType = T;
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
