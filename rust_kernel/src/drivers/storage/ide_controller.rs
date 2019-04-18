use super::{
    pci::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass},
    PciType0, PCI,
};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct PciIdeController {
    /*0        |*/ vendor_id: u16,
    /*2        |*/ device_id: u16,
    /*4        |*/ command: u16,
    /*6        |*/ status: u16,
    /*8        |*/ revision_id: u8,
    /*9        |*/ prog_if: u8,
    /*a        |*/ sub_class: u8,
    /*b        |*/ class_code: u8,
    /*c        |*/ cache_line_size: u8,
    /*d        |*/ latency_timer: u8,
    /*e        |*/ header_type: u8,
    /*f        |*/ bist: u8,
    /*10       |*/ primary_channel: IDEChannelRegisters,
    /*18       |*/ secondary_channel: IDEChannelRegisters,
    /*20       |*/ dma: u32,
    /*24       |*/ not_utilized: u32,
    /*28       |*/ cardbus_cis_pointer: u32,
    /*2c       |*/ subsystem_vendor_id: u16,
    /*2e       |*/ subsystem_id: u16,
    /*30       |*/ expansion_rom_base_address: u32,
    /*34       |*/ capabilities_pointer: u8,
    /*35       |*/ reserved: [u8; 7],
    /*3c       |*/ interrupt_line: u8,
    /*3d       |*/ interrupt_pin: u8,
    /*3e       |*/ min_grant: u8,
    /*3f       |*/ max_latency: u8,
}

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
pub struct IdeController {
    pci: PciType0,
    location: u32,
}

impl IdeController {
    pub fn init() -> Option<Self> {
        PCI.lock()
            .query_device(PciDeviceClass::MassStorageController(MassStorageControllerSubClass::IdeController(
                IdeControllerProgIf::IsaCompatibilityModeOnlyControllerBusMastered,
            )))
            .map(|(pci, location)| Self { pci, location })
    }
}
