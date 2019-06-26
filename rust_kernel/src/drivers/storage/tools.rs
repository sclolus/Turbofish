//! Sector and NbrSectors newtypes definitions: Convert methods from litteral u64.
//! This types are defined as usize:
//! In 32bits mode. we can address 2to max.
use super::{SECTOR_MASK, SECTOR_SHIFT};

use core::ops::{Add, Sub};

/// new type representing a number of sectors
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct NbrSectors(pub usize);

/// Convert a NbrSectors len into a litteral u64 len
impl Into<u64> for NbrSectors {
    fn into(self) -> u64 {
        self.0 as u64 >> SECTOR_SHIFT as u64
    }
}

/// Convert a NbrSectors len into a usize len
impl Into<usize> for NbrSectors {
    fn into(self) -> usize {
        (self.0 as usize) << SECTOR_SHIFT
    }
}

/// Convert a litteral u64 len into a NbrSectors len
impl From<u64> for NbrSectors {
    fn from(u: u64) -> Self {
        Self((u >> SECTOR_SHIFT as u64) as usize + if u & SECTOR_MASK as u64 != 0 { 1 } else { 0 })
    }
}

/// Convert a litteral usize len into a NbrSectors len
impl From<usize> for NbrSectors {
    fn from(u: usize) -> Self {
        Self((u >> SECTOR_SHIFT) + if u & SECTOR_MASK != 0 { 1 } else { 0 })
    }
}

/// Sub boilerplate for NbrSectors - NbrSectors
impl Sub<NbrSectors> for NbrSectors {
    type Output = Self;

    fn sub(self, other: NbrSectors) -> Self::Output {
        Self(self.0 - other.0)
    }
}

/// new type representing the start sector
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Sector(pub usize);

/// Convert a litteral u64 location into a Sector
impl From<u64> for Sector {
    fn from(u: u64) -> Self {
        Self((u >> SECTOR_SHIFT as u64) as _)
    }
}

/// Add boilerplate for Sector + NbrSectors
impl Add<NbrSectors> for Sector {
    type Output = Sector;

    fn add(self, other: NbrSectors) -> Self::Output {
        Self(self.0 + other.0)
    }
}
