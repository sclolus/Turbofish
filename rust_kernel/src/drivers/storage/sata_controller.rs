//! This module handle a SATA driver. See https://wiki.osdev.org/SATA, https://wiki.osdev.org/AHCI

use super::{Command, MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0, SerialAtaProgIf, PCI};

use crate::drivers::storage::tools::*;
use crate::memory::{get_physical_addr, tools::AllocFlags};
use alloc::vec::Vec;
use bit_field::BitField;
use core::mem::size_of;
use core::ptr::{read_volatile, write_volatile};

#[derive(Debug, Copy, Clone)]
pub enum SataError {
    /// no command slot available
    NoSlot,
    TaskFileError,
}

type Result<T> = core::result::Result<T, SataError>;

#[repr(C)]
struct AccessCmdTbl {
    cmdtbl: [CmdTbl; 32],
    data_poiters: [[*mut u8; NBR_PRDT_ENTRIES]; 32],
}

#[derive(Debug, Copy, Clone)]
#[repr(align(1024))]
#[repr(C)]
struct CmdList([CmdHeader; 32]);

#[repr(C)]
struct AccessPort {
    cmdlist: CustomBox<CmdList>,
    access_cmdtbl: CustomBox<AccessCmdTbl>,
    port: MemoryMapped<HbaPort>,
    received_fis: CustomBox<ReceivedFIS>,
}

pub struct SataController {
    pci: PciType0,
    location: u32,
    access_ports: Vec<AccessPort>,
}

const HBA_PxIS_TFES: u32 = (1 << 30);

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
        let (pci, location): (PciType0, u32) = PCI.lock().query_device(PciDeviceClass::MassStorageController(
            MassStorageControllerSubClass::SerialAta(SerialAtaProgIf::Ahci1),
        ))?;
        pci.set_command(PciCommand::BUS_MASTER, true, location);

        println!("bar 5: {:#X?}", pci.bar5);

        let hba_mem_cell = MemoryMapped::new(pci.bar5 as *mut HbaMem).unwrap();
        println!("{:#X?}", hba_mem_cell.inner);
        let hba_mem = hba_mem_cell.get();
        println!("number of cmd slots: {}", hba_mem.cap.get_number_of_command_slots());
        println!("{:#X?}", hba_mem);

        let mut vec = Vec::new();
        let virt = (pci.bar5 + 0x100) as *mut HbaPort;
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
        let access_ports = vec
            .into_iter()
            .map(|mut port| {
                let mut cmdlist: CustomBox<CmdList> =
                    CustomBox::new(unsafe { core::mem::zeroed() }, AllocFlags::CACHE_DISABLE);
                let mut access_cmdtbl: CustomBox<AccessCmdTbl> =
                    CustomBox::new(unsafe { core::mem::zeroed() }, AllocFlags::CACHE_DISABLE);
                let received_fis: CustomBox<ReceivedFIS> =
                    CustomBox::new(unsafe { core::mem::zeroed() }, AllocFlags::CACHE_DISABLE);
                let cmdtbl = &mut access_cmdtbl.as_mut().cmdtbl;

                for (cmdheader, cmdtable) in &mut cmdlist.as_mut().0.iter_mut().zip(cmdtbl.iter()) {
                    cmdheader.ctba = get_physical_addr((cmdtable as *const CmdTbl).into()).unwrap().0 as u32;
                    cmdheader.prdtl = NBR_PRDT_ENTRIES as u16;
                    // println!("{:?}", cmdheader);
                }
                unsafe {
                    write_volatile(
                        &mut ((*port.inner).clb) as *mut _,
                        get_physical_addr(cmdlist.ptr().into()).unwrap().0 as u32,
                    );
                    write_volatile(
                        &mut ((*port.inner).fb) as *mut _,
                        get_physical_addr(received_fis.ptr().into()).unwrap().0 as u32,
                    );
                }
                println!("{:#X?}", port.get());
                AccessPort { cmdlist, access_cmdtbl, port, received_fis }
            })
            .collect();
        Some(Self { pci, location, access_ports })
    }
    pub fn read(&mut self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> Result<usize> {
        unsafe { self.access_ports[0].read(start_sector, nbr_sectors, buf) }
    }
}

impl AccessPort {
    unsafe fn find_cmdslot(&self) -> Option<usize> {
        // If not set in SACT and CI, the slot is free
        let sact = read_volatile(&mut ((*self.port.inner).sact) as *mut _);
        let ci = read_volatile(&mut ((*self.port.inner).ci) as *mut _);
        let mut slots: u32 = sact | ci;
        for i in 0..32 {
            if (slots & 1) == 0 {
                return Some(i);
            }
            slots >>= 1;
        }
        None
    }
    // Find a free command list slot
    unsafe fn read(&mut self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> Result<usize> {
        write_volatile(&mut ((*self.port.inner).is) as *mut _, (-1_i32) as u32);
        let slot_index: usize = self.find_cmdslot().ok_or(SataError::NoSlot)?;
        let cmdheader: &mut CmdHeader = &mut self.cmdlist.as_mut().0[slot_index];
        cmdheader.flags.set_command_fis_length(size_of::<FisRegH2D>() / size_of::<u32>());
        cmdheader.flags.set_write(false);
        cmdheader.prdtl = NBR_PRDT_ENTRIES as u16;

        let size = nbr_sectors.into();
        let cmdtbl: &mut CmdTbl = &mut self.access_cmdtbl.as_mut().cmdtbl[slot_index];
        //TODO: handle multiple prd entry
        assert!(size <= 2 << 22);
        cmdtbl.prdt_entry[0].dba = get_physical_addr(buf.into()).unwrap().0 as u32;
        cmdtbl.prdt_entry[0].flags.set_byte_count(size);
        cmdtbl.prdt_entry[0].flags.set_interrupt_on_completion(true);
        // TODO: Last entry?

        let cmdfis: &mut FisRegH2D = &mut *(&mut cmdtbl.cfis as *mut _ as *mut u8 as *mut FisRegH2D);
        cmdfis.fis_type = FisType::FisTypeRegH2D as u8;
        cmdfis.flags.set_command(true);
        cmdfis.command = Command::AtaCmdWriteDmaExt as u8;
        cmdfis.device = 1 << 6; // LBA mode

        let lba_low = start_sector.0.get_bits(0..32) as u32;
        let lba_high = start_sector.0.get_bits(32..48) as u32;
        cmdfis.lba0 = lba_low.get_bits(0..8) as u8;
        cmdfis.lba1 = lba_low.get_bits(8..16) as u8;
        cmdfis.lba2 = lba_low.get_bits(16..24) as u8;
        cmdfis.lba3 = lba_low.get_bits(24..32) as u8;
        cmdfis.lba4 = lba_high.get_bits(0..8) as u8;
        cmdfis.lba5 = lba_high.get_bits(8..16) as u8;
        cmdfis.countl = nbr_sectors.0.get_bits(0..8) as u8;
        cmdfis.counth = nbr_sectors.0.get_bits(8..16) as u8;

        //TODO: wait port available
        write_volatile(&mut ((*self.port.inner).ci) as *mut _, (1 << slot_index) as u32); // Issue command
                                                                                          // Wait for completion
        loop {
            // In some longer duration reads, it may be helpful to spin on the DPS bit
            // in the PxIS port field as well (1 << 5)
            let ci = read_volatile(&mut ((*self.port.inner).ci) as *mut _); // Issue command
            if (ci & (1 << slot_index) as u32) == 0_u32 {
                break;
            }
            // Task file error
            let is = read_volatile(&mut ((*self.port.inner).is) as *mut _); // Issue command
            if (is & HBA_PxIS_TFES) != 0_u32 {
                return Err(SataError::TaskFileError);
            }
        }

        Ok(size)
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
    fn set_write(&mut self, w: bool) {
        self.0.set_bit(6, w);
    }
    /// command fis lenght in DWORD
    fn get_command_fis_length(&self) -> usize {
        self.0.get_bits(0..5) as usize
    }
    fn set_command_fis_length(&mut self, l: usize) {
        self.0.set_bits(0..5, l as u16);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct CmdHeader {
    // DW0
    flags: CmdHeaderFlags,
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

const NBR_PRDT_ENTRIES: usize = 56;

#[derive(Copy, Clone)]
#[repr(C)]
#[repr(align(1024))]
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

    /// size max 4Mo, size must be pair
    fn set_byte_count(&mut self, size: usize) {
        self.0.set_bits(0..22, size as u32 - 1);
    }

    fn set_interrupt_on_completion(&mut self, value: bool) {
        self.0.set_bit(31, value);
    }

    fn get_interrupt_on_completion(&self) -> bool {
        self.0.get_bit(31) as bool
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PrdtEntry {
    dba: u32,  // Data base address
    dbau: u32, // Data base address upper 32 bits
    rsv0: u32, // Reserved

    flags: PrdtFlags,
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct FisRegH2DFlags(u8);

impl FisRegH2DFlags {
    // u8  pmport:4;	// Port multiplier
    // u8  rsv0:3;		// Reserved
    // u8  c:1;		// 1: Command, 0: Control

    /// command fis lenght in DWORD
    fn get_command(&self) -> bool {
        self.0.get_bit(7)
    }
    fn set_command(&mut self, command: bool) {
        self.0.set_bit(7, command);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum FisType {
    FisTypeRegH2D = 0x27,   // Register FIS - host to device
    FisTypeRegD2H = 0x34,   // Register FIS - device to host
    FisTypeDmaAct = 0x39,   // DMA activate FIS - device to host
    FisTypeDmaSetup = 0x41, // DMA setup FIS - bidirectional
    FisTypeData = 0x46,     // Data FIS - bidirectional
    FisTypeBist = 0x58,     // BIST activate FIS - bidirectional
    FisTypePioSetup = 0x5F, // PIO setup FIS - device to host
    FisTypeDevBits = 0xA1,  // Set device bits FIS - device to host
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FisRegH2D {
    // DWORD 0
    fis_type: u8, // FIS_TYPE_REG_H2D
    flags: FisRegH2DFlags,

    command: u8,  // Command register
    featurel: u8, // Feature register, 7:0

    // DWORD 1
    lba0: u8,   // LBA low register, 7:0
    lba1: u8,   // LBA mid register, 15:8
    lba2: u8,   // LBA high register, 23:16
    device: u8, // Device register

    // DWORD 2
    lba3: u8,     // LBA register, 31:24
    lba4: u8,     // LBA register, 39:32
    lba5: u8,     // LBA register, 47:40
    featureh: u8, // Feature register, 15:8

    // DWORD 3
    countl: u8,  // Count register, 7:0
    counth: u8,  // Count register, 15:8
    icc: u8,     // Isochronous command completion
    control: u8, // Control register

    // DWORD 4
    rsv1: [u8; 4], // Reserved
}

#[derive(Debug, Copy, Clone)]
#[repr(align(256))]
#[repr(C)]
struct ReceivedFIS {
    // 0x00
    dsfis: FisDmaSetup, // DMA Setup FIS
    pad0: [u8; 4],

    // 0x20
    psfis: FisPioSetup, // PIO Setup FIS
    pad1: [u8; 12],

    // 0x40
    rfis: FisRegD2H, // Register – Device to Host FIS
    pad2: [u8; 4],

    // 0x58
    // sdbfis: FisDevBits, // Set Device Bit FIS
    fis_dev_bits: u8,
    fis_dev_bits_2: u8,

    // 0x60
    ufis: Ufis,

    // 0xA0
    rsv: Rsv,
}

define_raw_data!(Ufis, 64);
define_raw_data!(Rsv, 0x100 - 0xA0);

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct FisDmaSetupFlag(u8);

impl FisDmaSetupFlag {
    // u8  pmport:4;	// Port multiplier
    // u8  rsv0:1;		// Reserved
    // u8  d:1;		// Data transfer direction, 1 - device to host
    // u8  i:1;		// Interrupt bit
    // u8  a:1;            // Auto-activate. Specifies if DMA Activate FIS is needed
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FisDmaSetup {
    // DWORD 0
    fis_type: u8, // FIS_TYPE_DMA_SETUP

    flag: FisDmaSetupFlag,

    rsved: [u8; 2], // Reserved

    //DWORD 1&2
    dm_abuffer_id: u64, // DMA Buffer Identifier. Used to Identify DMA buffer in host memory. SATA Spec says host specific and not in Spec. Trying AHCI spec might work.

    //DWORD 3
    rsvd: u32, //More reserved

    //DWORD 4
    dm_abuf_offset: u32, //Byte offset into buffer. First 2 bits must be 0

    //DWORD 5
    transfer_count: u32, //Number of bytes to transfer. Bit 0 must be 0

    //DWORD 6
    resvd: u32, //Reserved
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FisPioSetup {
    // DWORD 0
    fis_type: u8, // FIS_TYPE_PIO_SETUP

    flags: u8,
    // u8  pmport:4;	// Port multiplier
    // u8  rsv0:1;		// Reserved
    // u8  d:1;		// Data transfer direction, 1 - device to host
    // u8  i:1;		// Interrupt bit
    // u8  rsv1:1;
    status: u8, // Status register
    error: u8,  // Error register

    // DWORD 1
    lba0: u8,   // LBA low register, 7:0
    lba1: u8,   // LBA mid register, 15:8
    lba2: u8,   // LBA high register, 23:16
    device: u8, // Device register

    // DWORD 2
    lba3: u8, // LBA register, 31:24
    lba4: u8, // LBA register, 39:32
    lba5: u8, // LBA register, 47:40
    rsv2: u8, // Reserved

    // DWORD 3
    countl: u8,   // Count register, 7:0
    counth: u8,   // Count register, 15:8
    rsv3: u8,     // Reserved
    e_status: u8, // New value of status register

    // DWORD 4
    tc: u16,       // Transfer count
    rsv4: [u8; 2], // Reserved
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct FisRegD2H {
    // DWORD 0
    fis_type: u8, // FIS_TYPE_REG_D2H

    flags: u8,
    // u8  pmport:4;    // Port multiplier
    // u8  rsv0:2;      // Reserved
    // u8  i:1;         // Interrupt bit
    // u8  rsv1:1;      // Reserved
    status: u8, // Status register
    error: u8,  // Error register

    // DWORD 1
    lba0: u8,   // LBA low register, 7:0
    lba1: u8,   // LBA mid register, 15:8
    lba2: u8,   // LBA high register, 23:16
    device: u8, // Device register

    // DWORD 2
    lba3: u8, // LBA register, 31:24
    lba4: u8, // LBA register, 39:32
    lba5: u8, // LBA register, 47:40
    rsv2: u8, // Reserved

    // DWORD 3
    countl: u8,    // Count register, 7:0
    counth: u8,    // Count register, 15:8
    rsv3: [u8; 2], // Reserved

    // DWORD 4
    rsv4: [u8; 4], // Reserved
}
