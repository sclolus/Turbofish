use crate::syscall::Errno;
use core::ops::{Add, Mul, Sub};
use num_traits::Num;

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
        Err(Errno::Ebadf)
    } else {
        Ok(x)
    }
}

#[inline(always)]
/// true if the num t is a multiple of on
pub fn is_aligned_on<T>(t: T, on: T) -> bool
where
    T: Copy + Num,
{
    t % on == T::zero()
}

#[inline(always)]
/// align the num t on the next multiple of on
pub fn align_next<T>(t: T, on: T) -> T
where
    T: Copy + Num,
{
    if is_aligned_on(t, on) {
        t
    } else {
        t + (on - (t % on))
    }
}

#[inline(always)]
/// align the num t on the next multiple of on
pub fn align_prev<T>(t: T, on: T) -> T
where
    T: Copy + Num,
{
    if is_aligned_on(t, on) {
        t
    } else {
        t - (t % on)
    }
}

/// Local Result structure
pub type IoResult<T> = core::result::Result<T, Errno>;
