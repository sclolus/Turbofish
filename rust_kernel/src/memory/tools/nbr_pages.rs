//! this module contains a new type `NbrPages` to represent a number of pages
use super::PAGE_SIZE;
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use try_clone_derive::TryClone;

/// new type representing a number of page
#[derive(Debug, Copy, Clone, TryClone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NbrPages(pub usize);

impl NbrPages {
    pub const _4K: NbrPages = NbrPages(1 << 0);
    pub const _8K: NbrPages = NbrPages(1 << 1);
    pub const _16K: NbrPages = NbrPages(1 << 2);
    pub const _32K: NbrPages = NbrPages(1 << 3);
    pub const _64K: NbrPages = NbrPages(1 << 4);
    pub const _128K: NbrPages = NbrPages(1 << 5);
    pub const _256K: NbrPages = NbrPages(1 << 6);
    pub const _512K: NbrPages = NbrPages(1 << 7);
    pub const _1MB: NbrPages = NbrPages(1 << 8);
    pub const _2MB: NbrPages = NbrPages(1 << 9);
    pub const _4MB: NbrPages = NbrPages(1 << 10);
    pub const _8MB: NbrPages = NbrPages(1 << 11);
    pub const _16MB: NbrPages = NbrPages(1 << 12);
    pub const _32MB: NbrPages = NbrPages(1 << 13);
    pub const _64MB: NbrPages = NbrPages(1 << 14);
    pub const _128MB: NbrPages = NbrPages(1 << 15);
    pub const _256MB: NbrPages = NbrPages(1 << 16);
    pub const _512MB: NbrPages = NbrPages(1 << 17);
    pub const _1GB: NbrPages = NbrPages(1 << 18);
    pub const _2GB: NbrPages = NbrPages(1 << 19);
    pub const _3GB: NbrPages = NbrPages((1 << 19) + (1 << 18));
    pub const _4GB: NbrPages = NbrPages(1 << 20);
}

impl NbrPages {
    /// convertion to number of bytes
    pub fn to_bytes(self) -> usize {
        self.into()
    }
}

impl From<usize> for NbrPages {
    /// convertion from number of bytes
    #[inline(always)]
    fn from(nb_bytes: usize) -> Self {
        Self(nb_bytes / PAGE_SIZE + (nb_bytes % PAGE_SIZE != 0) as usize)
    }
}

impl Into<usize> for NbrPages {
    /// convertion to number of bytes
    #[inline(always)]
    fn into(self) -> usize {
        self.0 * PAGE_SIZE as usize
    }
}

impl Add<NbrPages> for NbrPages {
    type Output = Self;
    fn add(self, rhs: NbrPages) -> Self {
        NbrPages(self.0 + rhs.0)
    }
}

impl Mul<usize> for NbrPages {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        NbrPages(self.0 * rhs)
    }
}

impl AddAssign<NbrPages> for NbrPages {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl SubAssign<NbrPages> for NbrPages {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Sub<NbrPages> for NbrPages {
    type Output = Self;
    fn sub(self, rhs: NbrPages) -> Self {
        NbrPages(self.0 - rhs.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_nb_pages() {
        let nb_pages: NbrPages = 8000.into();
        assert_eq!(nb_pages, NbrPages(2));
    }
}
