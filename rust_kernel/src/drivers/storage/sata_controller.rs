//! This module handle a SATA driver. See https://wiki.osdev.org/SATA, https://wiki.osdev.org/AHCI

use super::{MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0, SerialAtaProgIf, PCI};

use crate::drivers::storage::tools::*;
use alloc::vec::Vec;
use bit_field::BitField;

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
            .map(|(pci, location): (PciType0, u32)| {
                pci.set_command(PciCommand::BUS_MASTER, true, location);
                Self { pci, location }
            })
    }

    pub fn dump_hba(&self) {
        let hba_mem_cell = MemoryMapped::new(self.pci.bar5 as *mut HbaMem).unwrap();

        println!("{:#X?}", hba_mem_cell.inner);
        let hba_mem = hba_mem_cell.get();
        println!("number of cmd slots: {}", hba_mem.cap.get_number_of_command_slots());
        println!("{:#X?}", hba_mem);

        let mut vec = Vec::new();
        let virt = (self.pci.bar5 + 0x100) as *mut HbaPort;
        for i in 0..32 {
            if hba_mem.pi.get_bit(i) {
                let l = MemoryMapped::new(unsafe { virt.add(i) }).unwrap();
                let hba_port = l.get();
                if hba_port.sig == Self::SATA_SIG_ATA
                    || hba_port.sig == Self::SATA_SIG_ATAPI
                    || hba_port.sig == Self::SATA_SIG_SEMB
                    || hba_port.sig == Self::SATA_SIG_PM0
                {
                    vec.push(l);
                } else {
                    println!("invalid signature for port");
                }
            }
        }
        for p in vec {
            println!("{:#X?}", p.get());
        }
        println!("bar 5: {:#X?}", self.pci.bar5);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct Cap(u32);

impl Cap {
    fn get_number_of_command_slots(&self) -> usize {
        (self.0.get_bits(8..13) + 1) as usize
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct HbaMem {
    // 0x00 - 0x2B, Generic Host Control
    /*0        |*/ cap: Cap, // Host capability
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
#[repr(C)]
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

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct CmdHeaderFlags(u16);

impl CmdHeaderFlags {
    // cfl:5 :u8,		// Command FIS length in DWORDS, 2 ~ 16
    // a:1 :u8,		// ATAPI
    // w:1 :u8,		// Write, 1: H2D, 0: D2H
    // p:1 :u8,		// Prefetchable

    // r:1 :u8,		// Reset
    // b:1 :u8,		// BIST
    // c:1 :u8,		// Clear busy upon R_OK
    // rsv0:1 :u8,		// Reserved
    // pmp:4 :u8,		// Port multiplier port
    /// command fis lenght in DWORD
    fn get_command_fis_length(&self) -> usize {
        self.0.get_bits(0..5) as usize
    }
}

#[derive(Debug, Copy, Clone)]
struct CmdHeader {
    flags: CmdHeaderFlags,
    // DW0
    prdtl: u16, // Physical region descriptor table length in entries

    // DW1
    //VOLATILE !
    prdbc: u32, // Physical region descriptor byte count transferred

    // DW2, 3
    ctba: u32,  // Command table descriptor base address
    ctbau: u32, // Command table descriptor base address upper 32 bits

    // DW4 - 7
    rsv1: [u32; 4], // Reserved
}

const NBR_PRDT_ENTRIES: usize = 64;

struct CmdTbl {
    // 0x00
    cfis: [u8; 64], // Command FIS

    // 0x40
    acmd: [u8; 16], // ATAPI command, 12 or 16 bytes

    // 0x50
    rsv: [u8; 48], // Reserved

    // 0x80
    prdt_entry: [PrdtEntry; NBR_PRDT_ENTRIES], // Physical region descriptor table entries, 0 ~ 65535
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct PrdtFlags(u32);

impl PrdtFlags {
    // u32 dbc:22;		// Byte count, 4M max
    // u32 rsv1:9;		// Reserved
    // u32 i:1;		// Interrupt on completion
    /// command fis lenght in DWORD
    //TODO: check the 1..23
    fn get_byte_count(&self) -> usize {
        self.0.get_bits(0..22) as usize + 1
    }
    fn get_interrupt_on_completion(&self) -> bool {
        self.0.get_bit(31) as bool
    }
}

#[derive(Debug, Copy, Clone)]
struct PrdtEntry {
    dba: u32,  // Data base address
    dbau: u32, // Data base address upper 32 bits
    rsv0: u32, // Reserved

    flags: PrdtFlags,
}
