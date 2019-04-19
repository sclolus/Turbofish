use super::SECTOR_SIZE;
use core::ops::Add;

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
impl Add<NbrSectors> for Sector {
    type Output = Sector;

    fn add(self, other: NbrSectors) -> Self::Output {
        Self(self.0 + other.0)
    }
}

/// new type representing the start sector
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Sector(pub u64);
