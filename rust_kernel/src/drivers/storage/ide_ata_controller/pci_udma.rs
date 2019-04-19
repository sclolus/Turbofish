//! This module contains udma read/write methods on IDE drive

use super::{AtaResult, DmaIo, Drive};
use super::{NbrSectors, Sector};

impl DmaIo for Drive {
    /// drive specific READ method
    fn read(&self, _start_sector: Sector, _nbr_sectors: NbrSectors) -> AtaResult<()> {
        Ok(())
    }
    fn write(&self, _start_sector: Sector, _nbr_sectors: NbrSectors) -> AtaResult<()> {
        Ok(())
    }
}
