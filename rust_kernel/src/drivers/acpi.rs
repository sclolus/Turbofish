//! Minimal ACPI driver

#![deny(missing_docs)]

use crate::ffi::c_char;

use crate::memory::ffi::{map, unmap};

use core::mem::size_of;

use io::{Io, Pio};

use crate::drivers::PIT0;
use core::time::Duration;

use crate::Spinlock;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct RSDPDescriptor10 {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

/// ACPI version 2+ RSDP structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct RSDPDescriptor20 {
    legacy_part: RSDPDescriptor10,

    length: u32,
    xsdt_address_0_31: u32,
    xsdt_address_32_63: u32,
    extended_checksum: u8,
    reserved: [u8; 3],
}

/// ACPI version 1 RSDT structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct ACPIRSDTHeader {
    /*0  */ signature: [u8; 4],
    /*4  */ length: u32,
    /*8  */ revision: u8,
    /*9  */ checksum: u8,
    /*10 */ oemid: [u8; 6],
    /*16 */ oem_table_i_d: [u8; 8],
    /*24 */ oem_revision: u32,
    /*28 */ creator_id: u32,
    /*32 */ creator_revision: u32,
    /*36 */
}

/// ACPI version 2+ XSDT structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct ACPIXSDTHeader {
    /*0  */ signature: [u8; 4],
    /*4  */ length: u32,
    /*8  */ revision: u8,
    /*9  */ checksum: u8,
    /*10 */ oemid: [u8; 6],
    /*16 */ oem_table_i_d: [u8; 8],
    /*24 */ oem_revision: u32,
    /*28 */ creator_id: u32,
    /*32 */ creator_revision: u32,
    /*36 */
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct Rsdt {
    h: ACPIRSDTHeader,
    others_rsdt: *const Rsdt,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct Xsdt {
    h: ACPIXSDTHeader,
    next_xsdt_0_31: *const Xsdt,
    next_xsdt_low_32_63: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct GenericAddressStructure {
    address_space: u8,
    bit_width: u8,
    bit_offset: u8,
    access_size: u8,
    address_0_31: u32,
    address_32_63: u32,
}

/// [fadt](https://wiki.osdev.org/FADT)
/// Fixed ACPI Description Table
/// It is the fucking big table of ACPI (see acpidump)
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct FADT {
    /*0  */ h: ACPIRSDTHeader,
    /*36 */ firmware_ctrl: u32,
    /*40 */ dsdt: u32,

    // field used in ACPI 1.0, no longer in use, for compatibility only
    /*44 */
    reserved: u8,

    /*45 */ preferred_power_management_profile: u8,
    /*46 */ sci_interrupt: u16,
    /*48 */ smi_command_port: u32,
    /*52 */ acpi_enable: u8,
    /*53 */ acpi_disable: u8,
    /*54 */ s4_bios_req: u8,
    /*55 */ pstate_control: u8,
    /*56 */ pm1a_event_block: u32,
    /*60 */ pm1b_event_block: u32,
    /*64 */ pm1a_control_block: u32, // -> PM1a_cnt (ex: shutdow port 1)
    /*68 */ pm1b_control_block: u32, // -> PM1b_cnt (ex: shutdow port 2)
    pm2_control_block: u32,
    pm_timer_block: u32,
    gpe0_block: u32,
    gpe1_block: u32,
    pm1_event_length: u8,
    pm1_control_length: u8,
    pm2_control_length: u8,
    pm_timer_length: u8,
    gpe0_length: u8,
    gpe1_length: u8,
    gpe1_base: u8,
    c_state_control: u8,
    worst_c2_latency: u16,
    worst_c3_latency: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    century: u8,

    // reserved in ACPI 1.0, used since ACPI 2.0+
    boot_architecture_flags: u16,

    reserved2: u8,
    flags: u32,

    // 12 byte structure, see below for details
    reset_reg: GenericAddressStructure,

    reset_value: u8,
    reserved3: [u8; 3],

    // 64bit pointers - Available on ACPI 2.0+
    x_firmware_control_0_31: u32,
    x_firmware_control_32_63: u32,
    x_dsdt_0_31: u32,
    x_dsdt_32_63: u32,

    x_pm1a_event_block: GenericAddressStructure,
    x_pm1b_event_block: GenericAddressStructure,
    x_pm1a_control_block: GenericAddressStructure,
    x_pm1b_control_block: GenericAddressStructure,
    x_pm2_control_block: GenericAddressStructure,
    x_pm_timer_block: GenericAddressStructure,
    x_gpe0_block: GenericAddressStructure,
    x_gpe1_block: GenericAddressStructure,
}

/*
* Intel ACPI Component Architecture
* AML Disassembler version 20090123
*
* Disassembly of dsdt.aml, Mon May  6 20:41:40 2013
*
*
* Original Table Header:
*     Signature        "DSDT"
*     Length           0x00003794 (14228)
*     Revision         0x01 **** ACPI 1.0, no 64-bit math support
*     Checksum         0x46
*     OEM ID           "DELL"
*     OEM Table ID     "dt_ex"
*     OEM Revision     0x00001000 (4096)
*     Compiler ID      "INTL"
*     Compiler Version 0x20050624 (537200164)
*/
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct DsdtHeader {
    /*0 */ signature: [c_char; 4],
    /*4 */ length: u32,
    /*8 */ revision: u8,
    /*9 */ checksum: u8,
    /*10 */ oemid: [c_char; 6],
    /*16 */ oem_table_id: [c_char; 8],
    /*24 */ oem_revision: u32,
    /*28 */ compiler_id: [c_char; 4],
    /*32 */ compiler_version: u32,
}

//
// bytecode of the \_S5 object
// -----------------------------------------
//        | (optional) |    |    |    |
// NameOP | \          | _  | S  | 5  | _
// 08     | 5A         | 5F | 53 | 35 | 5F
//
// -----------------------------------------------------------------------------------------------------------
//           |           |              | ( SLP_TYPa   ) | ( SLP_TYPb   ) | ( Reserved   ) | (Reserved    )
// PackageOP | PkgLength | NumElements  | byteprefix Num | byteprefix Num | byteprefix Num | byteprefix Num
// 12        | 0A        | 04           | 0A         05  | 0A          05 | 0A         05  | 0A         05
//
//----this-structure-was-also-seen----------------------
// PackageOP | PkgLength | NumElements |
// 12        | 06        | 04          | 00 00 00 00
//
// (Pkglength bit 6-7 encode additional PkgLength bytes [shouldn't be the case here])
//
// PackageOP

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct S5Object {
    package_op: u8,
    pkg_length: u8,
    num_elements: u8,
    slp_typ_a_byteprefix: u8,
    slp_typ_a_num: u8,
    slp_typ_b_byteprefix: u8,
    slp_typ_b_num: u8,
}

/// Basics ACPI errors
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum AcpiError {
    AcpiAbsent,
    CannotInitialize,
    Disabled,
    Enabled,
    Timeout,
    InternalError,
    BadAcpiVersion,
}

/// Standard ACPI type result
pub type AcpiResult<T> = core::result::Result<T, AcpiError>;

/// Main driver structure
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct Acpi {
    rsdp_descriptor: RSDPDescriptor,
    fadt: FADT,
}

lazy_static! {
    /// ACPI driver
    pub static ref ACPI: Spinlock<Option<Acpi>> = Spinlock::new(None);
}

#[derive(Copy, Clone, Debug)]
enum RSDPDescriptor {
    LegacyRSDPDescriptor(RSDPDescriptor10),
    AdvancedRSDPDescriptor(RSDPDescriptor20),
}

const RSDP_BIOS_ADDR: *const u8 = 0xe0000 as *const u8;
const SLP_EN: u16 = 1 << 13;
const SLEEP_STATE_MAGIC_SHL: u16 = 10;

impl Acpi {
    /// Initialize the ACPI feature
    pub fn init() -> AcpiResult<()> {
        let rsdp_descriptor: RSDPDescriptor;
        let fadt;

        unsafe {
            let rsdp_descriptor_ptr: *const RSDPDescriptor10 = get_rsdp_descriptor_ptr()?;
            rsdp_descriptor = match (*rsdp_descriptor_ptr).revision {
                0 => RSDPDescriptor::LegacyRSDPDescriptor(*rsdp_descriptor_ptr),
                _ => RSDPDescriptor::AdvancedRSDPDescriptor(
                    *(rsdp_descriptor_ptr as *const RSDPDescriptor20),
                ),
            };

            let virt_addr: *const u8 = match rsdp_descriptor {
                RSDPDescriptor::LegacyRSDPDescriptor(descriptor) => map(
                    descriptor.rsdt_address as *mut u8,
                    size_of::<ACPIRSDTHeader>(),
                ),
                RSDPDescriptor::AdvancedRSDPDescriptor(descriptor) => map(
                    descriptor.xsdt_address_0_31 as *mut u8,
                    size_of::<ACPIRSDTHeader>(),
                ),
            };

            fadt = find_fadt(virt_addr as *const Rsdt);

            unmap(virt_addr as *mut u8, size_of::<ACPIRSDTHeader>());
        }
        match fadt {
            Ok(fadt) => {
                *ACPI.lock() = Some(Self {
                    rsdp_descriptor,
                    fadt,
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Enable ACPI
    pub fn enable(&mut self) -> AcpiResult<()> {
        // give 3 seconds for ACPI initialization timeout
        let mut count = 60;

        while count != 0 && self.is_disable() {
            Pio::<u8>::new(self.fadt.smi_command_port as u16).write(self.fadt.acpi_enable);
            PIT0.lock().sleep(Duration::from_millis(50));
            count -= 1;
        }

        if count != 0 {
            Ok(())
        } else {
            Err(AcpiError::Timeout)
        }
    }

    /// Check is ACPI is enable
    pub fn is_enable(&mut self) -> bool {
        Pio::<u16>::new(self.fadt.pm1a_control_block as u16).read() & 0x1 == 1
    }

    /// Disable ACPI
    pub fn disable(&mut self) -> AcpiResult<()> {
        // give 3 seconds for ACPI uninitialization timeout
        let mut count = 60;

        while count != 0 && self.is_enable() {
            Pio::<u8>::new(self.fadt.smi_command_port as u16).write(self.fadt.acpi_disable);
            PIT0.lock().sleep(Duration::from_millis(50));
            count -= 1;
        }

        if count != 0 {
            Ok(())
        } else {
            Err(AcpiError::Timeout)
        }
    }

    /// Check is ACPI is disable
    pub fn is_disable(&mut self) -> bool {
        Pio::<u16>::new(self.fadt.pm1a_control_block as u16).read() & 0x1 == 0
    }

    /// Perform a ACPI hard reboot
    pub fn reboot_computer(&mut self) -> AcpiResult<()> {
        if self.is_disable() {
            return Err(AcpiError::Disabled);
        }
        let acpi_revision = match self.rsdp_descriptor {
            RSDPDescriptor::LegacyRSDPDescriptor(descriptor) => descriptor.revision,
            RSDPDescriptor::AdvancedRSDPDescriptor(descriptor) => descriptor.legacy_part.revision,
        };
        if acpi_revision == 0 {
            return Err(AcpiError::BadAcpiVersion);
        }
        Pio::<u8>::new(self.fadt.reset_reg.address_0_31 as u16).write(self.fadt.reset_value);
        // Give 1 second for reboot before sending an error
        PIT0.lock().sleep(Duration::from_millis(1000));
        Err(AcpiError::InternalError)
    }

    /// Shutdown the computer now or return, result is not necessary
    /// Works only with ACPI2+ versions
    /// See: https://wiki.osdev.org/Reboot
    pub unsafe fn shutdown(&mut self) -> AcpiResult<()> {
        if self.is_disable() {
            return Err(AcpiError::Disabled);
        }

        let dsdt_header =
            map(self.fadt.dsdt as *mut u8, size_of::<DsdtHeader>()) as *const DsdtHeader;
        let dsdt = map(
            (self.fadt.dsdt as usize + size_of::<DsdtHeader>()) as *mut u8,
            (*dsdt_header).length as usize,
        ) as *const u8;

        // dbg!(*dsdt_header);

        let res = memschr(
            dsdt as *const u8,
            (*dsdt_header).length as usize - size_of::<DsdtHeader>(),
            "_S5_".as_ptr(),
            4,
            None,
        )
        .map(|pdsdt| {
            let s5_obj = *(pdsdt.add(4) as *const S5Object);

            if s5_obj.package_op == 0x12 {
                // disable all interrupts
                interrupts::disable();

                // println!(
                //    "ports: A -> {:#X?} B -> {:#X?}",
                //    self.fadt.pm1a_control_block, self.fadt.pm1b_control_block
                // );
                // println!("SLP_TYPa: {:#X?}", s5_obj.slp_typ_a_num);
                // println!("SLP_TYPb: {:#X?}", s5_obj.slp_typ_b_num);

                println!("preparing shutdown...");
                Pio::<u16>::new(self.fadt.pm1a_control_block as u16)
                    .write(((s5_obj.slp_typ_a_num as u16) << SLEEP_STATE_MAGIC_SHL) | SLP_EN);
                if self.fadt.pm1b_control_block != 0 {
                    Pio::<u16>::new(self.fadt.pm1b_control_block as u16)
                        .write(((s5_obj.slp_typ_b_num as u16) << SLEEP_STATE_MAGIC_SHL) | SLP_EN);
                }
                // wait for shutdown in iddle mode
                asm!("hlt");
            }
        });
        // _S5_ instructions OR s5_obj.package_op not found !
        unmap(dsdt as *mut u8, (*dsdt_header).length as usize);
        unmap(dsdt_header as *mut u8, size_of::<DsdtHeader>());
        res
    }
}

/// Get the first main ACPI descriptor
unsafe fn get_rsdp_descriptor_ptr() -> AcpiResult<*const RSDPDescriptor10> {
    match memschr(
        RSDP_BIOS_ADDR,
        0x20000,
        "RSD PTR ".as_ptr(),
        8,
        Some(rsdp_checksum),
    ) {
        Ok(rsdp_descriptor_ptr) => Ok(rsdp_descriptor_ptr as *const RSDPDescriptor10),
        Err(_) => Err(AcpiError::AcpiAbsent),
    }
}

/// Search the big fucking table
unsafe fn find_fadt(rsdt: *const Rsdt) -> AcpiResult<FADT> {
    let entries = (((*rsdt).h.length - size_of::<ACPIRSDTHeader>() as u32) / 4) as usize;

    let others_rsdt = map(
        (*rsdt).others_rsdt as *mut u8,
        size_of::<ACPIRSDTHeader>() * entries,
    );

    // println!("begin research... on {:?}", others_rsdt);
    for i in 0..entries {
        let h = (others_rsdt as *const ACPIRSDTHeader).add(i);

        if (*h).signature == "FACP".as_bytes() {
            // println!("iteration {} / {}: sign = ", i, entries);
            // println!("{:?}", (*h).signature);
            let ret = *(h as *const FADT);
            unmap(
                others_rsdt as *mut u8,
                size_of::<ACPIRSDTHeader>() * entries,
            );
            return Ok(ret);
        }
    }
    // No FACP found
    unmap(
        others_rsdt as *mut u8,
        size_of::<ACPIRSDTHeader>() * entries,
    );
    Err(AcpiError::CannotInitialize)
}

/// Checksum for rsdp descriptor
unsafe fn rsdp_checksum(rsdp_descriptor: *const u8) -> bool {
    let ptr: *const u8 = rsdp_descriptor;
    let mut checksum: u8 = 0;

    for i in 0..size_of::<RSDPDescriptor10>() {
        checksum = checksum.overflowing_add(*ptr.add(i)).0;
    }

    if checksum == 0 {
        true
    } else {
        false
    }
}

/// Search a sized pattern in a designed memory area
unsafe fn memschr(
    mut base_mem: *const u8,
    range: usize,
    expr: *const u8,
    len_expr: usize,
    contraint: Option<unsafe fn(*const u8) -> bool>,
) -> AcpiResult<*const u8> {
    if len_expr == 0 {
        return Err(AcpiError::InternalError);
    }
    // Be careful with overflow
    if base_mem as usize > u32::max_value() as usize - range {
        return Err(AcpiError::InternalError);
    }
    let end_mem = base_mem.add(range);
    while base_mem.add(len_expr) < end_mem {
        let mut len: usize = 0;
        while *base_mem.add(len) == *expr.add(len) && len < len_expr {
            len += 1;
        }
        if len == len_expr {
            if let Some(contraint) = contraint {
                if contraint(base_mem) {
                    return Ok(base_mem);
                }
            } else {
                return Ok(base_mem);
            }
        }
        base_mem = base_mem.add(1);
    }
    return Err(AcpiError::InternalError);
}
