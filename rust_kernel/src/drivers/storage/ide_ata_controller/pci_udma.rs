use super::{
    AtaResult, DmaIo, Drive, IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, PCI,
};
use crate::drivers::storage::tools::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct IDEChannelRegisters {
    /// I/O base
    base: u16,
    /// Control Base
    ctrl: u16,
    /// Bus Master IDE
    bus_master_ide: u16,
    /// nIEN (No Interrupt) ???
    raw_stuff: [u8; 2],
}

impl DmaIo for Drive {
    /// drive specific READ method
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> AtaResult<()> {
        Ok(())
    }
    fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        Ok(())
    }
}
