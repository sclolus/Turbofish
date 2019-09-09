use core::ops::{Add, Mul, Sub};
use libc_binding::Errno;

/// The Ext2 file system divides up disk space into logical blocks of contiguous space.
/// The size of blocks need not be the same size as the sector size of the disk the file system resides on.
/// The size of blocks can be determined by reading the field starting at byte 24 in the Superblock.
#[derive(Debug, Copy, Clone, PartialEq, Default, PartialOrd)]
#[repr(transparent)]
pub struct Block(pub u32);

/// Roundup style function
pub fn div_rounded_up(a: u64, b: u64) -> u64 {
    (a + b - 1) / b
}

/// Add boilerplate for Block
impl Add<Self> for Block {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Self> for Block {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<u32> for Block {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

/// return an error if block x is 0
pub fn err_if_zero(x: Block) -> Result<Block, Errno> {
    if x == Block(0) {
        Err(Errno::EBADF)
    } else {
        Ok(x)
    }
}

#[inline(always)]
/// align the num t on the next multiple of on
pub fn align_prev(t: u64, on: u64) -> u64 {
    debug_assert!(on.is_power_of_two());
    if t & (on - 1) == 0 {
        t
    } else {
        t - (t & (on - 1))
    }
}

#[inline(always)]
pub fn align_next(t: u64, on: u64) -> u64 {
    debug_assert!(on.is_power_of_two());
    if t & (on - 1) == 0 {
        t
    } else {
        t + (on - (t & (on - 1)))
    }
}

#[inline(always)]
/// align the num t on the next multiple of on
pub fn u32_align_prev(t: u32, on: u32) -> u32 {
    debug_assert!(on.is_power_of_two());
    if t & (on - 1) == 0 {
        t
    } else {
        t - (t & (on - 1))
    }
}

#[inline(always)]
pub fn u32_align_next(t: u32, on: u32) -> u32 {
    debug_assert!(on.is_power_of_two());
    if t & (on - 1) == 0 {
        t
    } else {
        t + (on - (t & (on - 1)))
    }
}
/// Local Result structure
pub type IoResult<T> = core::result::Result<T, Errno>;
