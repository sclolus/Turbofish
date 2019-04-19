use super::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, PCI};

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

#[derive(Copy, Clone, Debug)]
pub struct PciUdma {
    pci: PciType0,
    location: u32,
}

impl PciUdma {
    pub fn init() -> Option<Self> {
        PCI.lock()
            .query_device(PciDeviceClass::MassStorageController(MassStorageControllerSubClass::IdeController(
                IdeControllerProgIf::IsaCompatibilityModeOnlyControllerBusMastered,
            )))
            .map(|(pci, location)| Self { pci, location })
    }
}
