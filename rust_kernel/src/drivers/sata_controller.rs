use super::{
    pci::{MassStorageControllerSubClass, PciDeviceClass, SerialAtaProgIf},
    PCI,
};

use super::PciType0;

#[derive(Copy, Clone, Debug)]
pub struct SataController {
    pci: PciType0,
    location: u32,
}

impl SataController {
    pub fn init() -> Option<Self> {
        PCI.lock()
            .query_device(PciDeviceClass::MassStorageController(MassStorageControllerSubClass::SerialAta(
                SerialAtaProgIf::Ahci1,
            )))
            .map(|(pci, location)| Self { pci, location })
    }
}
