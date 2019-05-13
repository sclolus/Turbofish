use core::ops::{Add, Mul};

/// The Ext2 file system divides up disk space into logical blocks of contiguous space.
/// The size of blocks need not be the same size as the sector size of the disk the file system resides on.
/// The size of blocks can be determined by reading the field starting at byte 24 in the Superblock.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
pub struct Block(pub u32);

/// Roundup style function
pub fn div_rounded_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

/// Add boilerplate for Block
impl Add<Self> for Block {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<u32> for Block {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

pub fn err_if_zero(x: Block) -> Result<Block, IoError> {
    if x == Block(0) {
        Err(IoError::FileOffsetOutOfFile)
    } else {
        Ok(x)
    }
}

/// Local Result structure
pub type IoResult<T> = core::result::Result<T, IoError>;

/// Local Error structure
#[derive(Debug, Copy, Clone)]
pub enum IoError {
    NoSuchFileOrDirectory,
    FileOffsetOutOfFile,
    NotADirectory,
    NoSpaceLeftOnDevice,
    FilenameTooLong,
    EndOfFile,
    InodeNotValid,
}
