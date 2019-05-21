//! This module handle a SATA driver. See https://wiki.osdev.org/SATA, https://wiki.osdev.org/AHCI

use super::{MassStorageControllerSubClass, PciDeviceClass, PciType0, SerialAtaProgIf, PCI};

use crate::memory::allocator::{map, unmap};
use core::mem::size_of;
use raw_data::define_raw_data;

use alloc::vec::Vec;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct HbaMem {
    // 0x00 - 0x2B, Generic Host Control
    /*0        |*/ cap: u32, // Host capability
    /*4        |*/ ghc: u32, // Global host control
    /*8        |*/ is: u32, // Interrupt status
    /*c        |*/ pi: u32, // Port implemented
    /*10       |*/ vs: u32, // Version
    /*14       |*/ ccc_ctl: u32, // Command completion coalescing control
    /*18       |*/ ccc_pts: u32, // Command completion coalescing ports
    /*1c       |*/ en_loc: u32, // Enclosure management location
    /*20       |*/ en_ctl: u32, // Enclosure management control
    /*24       |*/ cap2: u32, // Host capabilities extended
    /*28       |*/ bohc: u32, // BIOS/OS handoff control and status

    // 0x2C - 0x9F, Reserved
    /*2C       |*/ reserved: Reserved,

    // 0xA0 - 0xFF, Vendor specific registers
    /*A0       |*/
    vendor_specific_registers: VendorSpecificRegisters,
    // 0x100 - 0x10FF, Port control registers ... (relative to pi value)
}

define_raw_data!(Reserved, 0xA0 - 0x2C);
define_raw_data!(VendorSpecificRegisters, 0x100 - 0xA0);

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct HbaPort {
    /*0        |*/ clb: u32, // command list base address, 1K-byte aligned
    /*4        |*/ clbu: u32, // command list base address upper 32 bits
    /*8        |*/ fb: u32, // FIS base address, 256-byte aligned
    /*c        |*/ fbu: u32, // FIS base address upper 32 bits
    /*10       |*/ is: u32, // interrupt status
    /*14       |*/ ie: u32, // interrupt enable
    /*18       |*/ cmd: u32, // command and status
    /*1c       |*/ rsv0: u32, // Reserved
    /*20       |*/ tfd: u32, // task file data
    /*24       |*/ sig: u32, // signature
    /*28       |*/ ssts: u32, // SATA status (SCR0:SStatus)
    /*2c       |*/ sctl: u32, // SATA control (SCR2:SControl)
    /*30       |*/ serr: u32, // SATA error (SCR1:SError)
    /*34       |*/ sact: u32, // SATA active (SCR3:SActive)
    /*38       |*/ ci: u32, // command issue
    /*3c       |*/ sntf: u32, // SATA notification (SCR4:SNotification)
    /*40       |*/ fbs: u32, // FIS-based switch control

    // 0x44 ~ 0x6F, Reserved
    /*10       |*/ reserved: ReservedPort,

    // 0x70 ~ 0x7F, vendor specific
    /*10       |*/ vendor_specific: VendorSpecificPort,
}

define_raw_data!(ReservedPort, 0x70 - 0x44);
define_raw_data!(VendorSpecificPort, 0x80 - 0x70);

#[derive(Copy, Clone, Debug)]
pub struct SataController {
    pci: PciType0,
    location: u32,
}

impl SataController {
    /// SATA drive
    const SATA_SIG_ATA: u32 = 0x00000101;
    /// SATAPI drive
    const SATA_SIG_ATAPI: u32 = 0xEB140101;
    /// Enclosure management bridge
    const SATA_SIG_SEMB: u32 = 0xC33C0101;
    /// Port multiplier
    const SATA_SIG_PM0: u32 = 0x96690101;

    pub fn init() -> Option<Self> {
        PCI.lock()
            .query_device(PciDeviceClass::MassStorageController(MassStorageControllerSubClass::SerialAta(
                SerialAtaProgIf::Ahci1,
            )))
            .map(|(pci, location)| Self { pci, location })
    }

    pub fn dump_hba(&self) {
        let virt = unsafe { map(self.pci.bar5 as *mut u8, size_of::<HbaMem>()) };
        println!("{:#X?}", virt);

        let s = virt as *mut HbaMem;
        let mut vec = Vec::new();
        unsafe {
            println!("{:#X?}", *s);
            let virt =
                map((self.pci.bar5 + 0x100) as *mut u8, size_of::<HbaPort>() * (*s).pi as usize) as *const HbaPort;
            for i in 0..(*s).pi as usize {
                let l = core::ptr::read_volatile(virt.add(i));
                if l.sig == Self::SATA_SIG_ATA
                    || l.sig == Self::SATA_SIG_ATAPI
                    || l.sig == Self::SATA_SIG_SEMB
                    || l.sig == Self::SATA_SIG_PM0
                {
                    vec.push(virt.add(i));
                }
            }
            for h in vec {
                println!("{:#X?}", core::ptr::read_volatile(h));
            }
            unmap(virt as *mut u8, size_of::<HbaPort>() * (*s).pi as usize);
        }
        unsafe {
            unmap(virt, size_of::<HbaMem>());
        }
        println!("bar 5: {:#X?}", self.pci.bar5);
    }
}
