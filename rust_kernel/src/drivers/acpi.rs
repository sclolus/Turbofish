// ACPI version 1 RSDP structure
use crate::ffi::c_char;
use crate::memory::allocator::KERNEL_VIRTUAL_PAGE_ALLOCATOR;
use crate::memory::tools::address::Address;
use crate::memory::tools::{Phys, Virt};
use core::mem::size_of;

use io::{Io, Pio};

use crate::drivers::pit_8253::PIT0;
use core::time::Duration;

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
struct RSDPDescriptor {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

// ACPI version 2+ RSDP structure
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct RSDPDescriptor20 {
    first_part: RSDPDescriptor,

    length: u32,
    xsdt_address_0_31: u32,
    xsdt_address_32_63: u32,
    extended_checksum: u8,
    reserved: [u8; 3],
}

// ACPI version 1 RSDT structure
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

// ACPI version 2+ XSDT structure
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

///[fadt](https://wiki.osdev.org/FADT)
///Fixed ACPI Description Table
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct FADT {
    /*0  */ h: ACPIRSDTHeader,
    /*36 */ firmware_ctrl: u32,
    /*40 */ dsdt: u32,

    // field used in ACPI 1.0, no longer in use, for compatibility only
    /*44 */ reserved: u8,

    /*45 */ preferred_power_management_profile: u8,
    /*46 */ sci_interrupt: u16,
    /*48 */ smi_command_port: u32,
    /*52 */ acpi_enable: u8,
    /*53 */ acpi_disable: u8,
    /*54 */ s4_bios_req: u8,
    /*55 */ pstate_control: u8,
    /*56 */ pm1a_event_block: u32,
    /*60 */ pm1b_event_block: u32,
    /*64 */ pm1a_control_block: u32, // -> PM1a_cnt (ex: shutdow port 1)           // SLP_EN is the bit 13
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

fn map_helper(phy_addr: *mut u8, mut size: usize) -> *mut u8 {
    let offset = Phys(phy_addr as usize).offset();
    if offset != 0 {
        size += 4096;
    }
    unsafe {
        let virt_addr = KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .map_addr(Phys(phy_addr as usize).into(), size.into())
            .unwrap();
        (virt_addr.to_addr().0 + offset) as *mut u8
    }
}

fn unmap_helper(virt_addr: *mut u8, mut size: usize) {
    if Virt(virt_addr as usize).offset() != 0 {
        size += 4096;
    }
    unsafe {
        KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .unmap_addr(Virt(virt_addr as usize).into(), size.into())
            .unwrap();
    }
}

pub unsafe fn acpi() -> Result<(), ()> {
    let rsdp_descriptor: *const RSDPDescriptor = rdsp_stage()?;

    println!("{:#p}", (*rsdp_descriptor).rsdt_address as *mut u8);

    if (*rsdp_descriptor).revision == 0 {
        // legacy RSDP descriptor
        let v = map_helper((*rsdp_descriptor).rsdt_address as *mut u8, size_of::<ACPIRSDTHeader>());

        rsdt_stage(v as *const ACPIRSDTHeader).unwrap();

        unmap_helper(v, size_of::<ACPIRSDTHeader>());
    } else {
        // extended RSDP descriptor
        let v = map_helper(
            (*(rsdp_descriptor as *const RSDPDescriptor20)).xsdt_address_0_31 as *mut u8,
            size_of::<ACPIRSDTHeader>(),
        );

        rsdt_stage(v as *const ACPIRSDTHeader).unwrap();

        unmap_helper(v, size_of::<ACPIRSDTHeader>());
    }
    Ok(())
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

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
struct S5Object {
    package_op: u8,
    pkg_length: u8,
    num_elements: u8,
    slp_typ_a_byteprefix: u8,
    slp_typ_a_num: u8,
    slp_typ_b_byteprefix: u8,
    slp_typ_b_num: u8,
}

const SLP_EN: u16 = 1 << 13;

unsafe fn acpi_enable(fadt: *const FADT) {
    println!("initial_state: {:?}", Pio::<u16>::new((*fadt).pm1a_control_block as u16).read());

    while Pio::<u16>::new((*fadt).pm1a_control_block as u16).read() & 0x1 != 1 {
        Pio::<u8>::new((*fadt).smi_command_port as u16).write((*fadt).acpi_enable);
    }
    println!("final state: {:?}", Pio::<u16>::new((*fadt).pm1a_control_block as u16).read());
    PIT0.lock().sleep(Duration::from_millis(200));
}

unsafe fn rsdt_stage(acpi_rsdt_header: *const ACPIRSDTHeader) -> Result<S5Object, ()> {
    println!("start rsdt_stage at {:#X?}", acpi_rsdt_header);

    let fadt: *const FADT = find_facp(acpi_rsdt_header as *const Rsdt)?;

    acpi_enable(fadt);

    let dsdt_header = map_helper((*fadt).dsdt as *mut u8, size_of::<DsdtHeader>()) as *const DsdtHeader;

    let dsdt = map_helper(((*fadt).dsdt as usize + size_of::<DsdtHeader>()) as *mut u8, (*dsdt_header).length as usize)
        as *const u8;

    dbg!(*dsdt_header);

    let pdsdt =
        memschr(dsdt as *const u8, (*dsdt_header).length as usize - size_of::<DsdtHeader>(), "_S5_".as_ptr(), 4, None)?;

    let s5_obj = pdsdt.add(4) as *const S5Object;

    if (*s5_obj).package_op == 0x12 {
        let ret = *s5_obj;
        unmap_helper(dsdt as *mut u8, (*dsdt_header).length as usize);
        unmap_helper(dsdt_header as *mut u8, size_of::<DsdtHeader>());

        println!("ports: A -> {:#X?} B -> {:#X?}", (*fadt).pm1a_control_block, (*fadt).pm1b_control_block,);
        println!("SLP_TYPa: {:#X?}", ret.slp_typ_a_num);
        println!("SLP_TYPb: {:#X?}", ret.slp_typ_b_num);
        println!("{:#X?}", ret);

        asm!("cli" :::: "volatile");
        println!("preparing shutdowm");
        Pio::<u16>::new((*fadt).pm1a_control_block as u16).write(((ret.slp_typ_a_num as u16) << 10) | SLP_EN);
        if (*fadt).pm1b_control_block != 0 {
            Pio::<u16>::new((*fadt).pm1b_control_block as u16).write(((ret.slp_typ_b_num as u16) << 10) | SLP_EN);
        }
        asm!("hlt");
        Ok(ret)
    } else {
        unmap_helper(dsdt as *mut u8, (*dsdt_header).length as usize);
        unmap_helper(dsdt_header as *mut u8, size_of::<DsdtHeader>());
        Err(())
    }

    // println!("{:?}", fadt);
    // println!("founded state: {:?}\n", fadt);
    // println!("flags {:x?} for IA BootArchitectureFlags\n", (*fadt).boot_architecture_flags);
    // if ((*fadt).boot_architecture_flags & 0x2) != 0 {
    //    println!("8042 founded !");
    // } else {
    //    println!("8042 not founded ! :(((");
    // }
}

unsafe fn find_facp(rsdt: *const Rsdt) -> Result<*const FADT, ()> {
    println!("{}", function!());
    let entries = (((*rsdt).h.length - size_of::<ACPIRSDTHeader>() as u32) / 4) as usize;

    let others_rsdt = map_helper((*rsdt).others_rsdt as *mut u8, size_of::<ACPIRSDTHeader>() * entries);

    println!("begin research... on {:?}", others_rsdt);
    for i in 0..entries {
        let h = (others_rsdt as *const ACPIRSDTHeader).add(i);

        if (*h).signature == "FACP".as_bytes() {
            println!("iteration {} / {}: sign = ", i, entries);
            println!("{:?}", (*h).signature);
            return Ok(h as *const FADT);
        }
    }
    // No FACP found
    Err(())
}

unsafe fn rdsp_stage() -> Result<*const RSDPDescriptor, ()> {
    let bios_addr = 0xe0000 as *const u8;

    let rsdp_descriptor =
        memschr(bios_addr, 0x20000, "RSD PTR ".as_ptr(), 8, Some(rsdp_checksum))? as *const RSDPDescriptor;

    println!("ACPI RSDP_DESCRIPTOR founded !\n");
    println!("rsdp descriptor: {:?}", *rsdp_descriptor);

    Ok(rsdp_descriptor)
}

// checksum for rsdp descriptor
unsafe fn rsdp_checksum(rsdp_descriptor: *const u8) -> bool {
    let ptr: *const u8 = rsdp_descriptor;
    let checksum: u8 = 0;

    for i in 0..size_of::<RSDPDescriptor>() {
        checksum.overflowing_add(*ptr.add(i));
    }

    if checksum == 0 {
        true
    } else {
        false
    }
}

// search a sized pattern in a designed memory area
unsafe fn memschr(
    mut base_mem: *const u8,
    range: usize,
    expr: *const u8,
    len_expr: usize,
    contraint: Option<unsafe fn(*const u8) -> bool>,
) -> Result<*const u8, ()> {
    if len_expr == 0 {
        return Err(());
    }
    // Be careful with overflow
    if base_mem as usize > u32::max_value() as usize - range {
        return Err(());
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
    return Err(());
}
