//! This module contains the turbo fish's storage drivers
#[deny(missing_docs)]
use super::pci::{
    IdeControllerProgIf, MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0, SerialAtaProgIf, PCI,
};

const SECTOR_SIZE: usize = 512;

pub mod ide_ata_controller;
pub use ide_ata_controller::IdeAtaController;

pub mod sata_controller;
pub use sata_controller::SataController;

pub mod tools;
pub use tools::{NbrSectors, Sector};

#[allow(dead_code)]
#[repr(u8)]
pub enum Command {
    AtaCmdReadPio = 0x20,
    AtaCmdReadPioExt = 0x24,
    AtaCmdReadDma = 0xC8,
    AtaCmdReadDmaExt = 0x25,
    AtaCmdWritePio = 0x30,
    AtaCmdWritePioExt = 0x34,
    AtaCmdWriteDma = 0xCA,
    AtaCmdWriteDmaExt = 0x35,
    AtaCmdCacheFlush = 0xE7,
    AtaCmdCacheFlushExt = 0xEA,
    AtaCmdPacket = 0xA0,
    AtaCmdIdentifyPacket = 0xA1,
    AtaCmdIdentify = 0xEC,
}
