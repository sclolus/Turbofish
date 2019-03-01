use super::PAGE_DIRECTORY;
use super::PAGE_SIZE;
use bit_field::BitField;
use core::ops::{Range, RangeInclusive};

pub trait Address: From<usize> + Into<usize> + Copy + Clone + Ord + Eq {
    #[inline(always)]
    fn page_is_aligned(&self) -> bool {
        let addr: usize = (*self).into();

        (addr & (PAGE_SIZE - 1)) == 0
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtualAddr(pub usize);

impl VirtualAddr {
    pub fn physical_addr(&self) -> Option<PhysicalAddr> {
        let page_directory_index = self.pd_index();
        let page_table_index = self.pt_index();
        //     let page_directory = unsafe { &*PageDirectory::get_current_page_directory() };

        let page_table = unsafe { &*PAGE_DIRECTORY[page_directory_index].get_page_table()? };

        if page_table[page_table_index].present() {
            Some(page_table[page_table_index].physical_address().into())
        } else {
            None
        }
    }

    #[inline]
    pub fn pd_index(&self) -> usize {
        self.0.get_bits(22..32)
    }

    #[inline]
    pub fn pt_index(&self) -> usize {
        self.0.get_bits(12..22)
    }

    #[inline]
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

impl Address for PhysicalAddr {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Page<T: Address> {
    pub number: usize,
    _phantom: core::marker::PhantomData<T>,
}

// impl<T: Address> Into<T> for Page<T> {
//     fn into(self) -> T {
//         let number: usize = self.number.into();
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

    pub fn into_addr(self) -> T {
        let number: usize = self.number.into();
        (number * PAGE_SIZE).into()
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
        let convert_addr: PhysicalAddr = page.into_addr();
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
}
