//! This module contains udma read/write methods on IDE drive

use super::Udma;
use super::{AtaResult, DmaIo, Drive};
use super::{NbrSectors, Sector};

impl DmaIo for Drive {
    /// drive specific READ method
    fn read(&self, _start_sector: Sector, _nbr_sectors: NbrSectors, _buf: *mut u8, _udma: &mut Udma) -> AtaResult<()> {
        Ok(())
    }
    fn write(
        &self,
        _start_sector: Sector,
        _nbr_sectors: NbrSectors,
        _buf: *const u8,
        _udma: &mut Udma,
    ) -> AtaResult<()> {
        Ok(())
    }
}
