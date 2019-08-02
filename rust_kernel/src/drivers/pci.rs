//! See [PCI](https://wiki.osdev.org/PCI)
use crate::Spinlock;

use core::mem::{size_of, transmute_copy};
use io::{Io, Pio};
use lazy_static::lazy_static;

use bit_field::BitField;
use bitflags::bitflags;

pub struct Pci {
    pub devices_list: CustomPciDeviceAllocator,
}

lazy_static! {
    pub static ref PCI: Spinlock<Pci> = Spinlock::new(Pci::new());
}

/// That Rust macro extend code of lot of PIO calls
macro_rules! fill_struct_with_io {
            ($(#[$e:meta])*
                struct $name:ident {
                $($field:ident : $type:ty,)*
                })  => {
                #[derive(Default)]
                $(#[$e])*
                struct $name {
                    $($field : $type,)*
                }
                impl $name {
                    fn fill(base_location: u32) -> Self {
                        let mut s: Self = Default::default();
                        $(
                            let location = base_location + (&s.$field as *const $type as u32 - &s as *const Self as u32);
                            Pio::<u32>::new(Pci::CONFIG_ADDRESS).write(location);
                            s.$field = Pio::<$type>::new(Pci::CONFIG_DATA).read();
                        )*
                        s
                    }
                }
            }
}

// Rust abstract of First line of PCI header
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceHeaderL0Raw {
        l0: u32,
    }
);

// Rust Abstract of next third lines of PCI header
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceHeaderBodyRaw {
        l1: u32,
        l2: u32,
        l3: u32,
    }
);

// Rust Abstract of Pci Registers (body)
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceRegistersRaw {
        l4: u32,
        l5: u32,
        l6: u32,
        l7: u32,
        l8: u32,
        l9: u32,
        l10: u32,
        l11: u32,
        l12: u32,
        l13: u32,
        l14: u32,
        l15: u32,
    }
);

// Rust Abstract of Complete Pci Device
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceRaw {
        l0: u32,
        l1: u32,
        l2: u32,
        l3: u32,
        l4: u32,
        l5: u32,
        l6: u32,
        l7: u32,
        l8: u32,
        l9: u32,
        l10: u32,
        l11: u32,
        l12: u32,
        l13: u32,
        l14: u32,
        l15: u32,
    }
);

/// Pci Header. 0x0 => 0x4
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PciDeviceHeaderL0 {
    /*0        |*/ vendor_id: u16,
    /*2        |*/ device_id: u16,
    /*4        |*/
}

/// Pci Header. 0x4 => 0x10
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PciDeviceHeaderBody {
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
    /*10       |*/
}

/// This table is applicable if the Header Type is 00h
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PciDeviceType0 {
    /*10       |*/ bar0: u32,
    /*14       |*/ bar1: u32,
    /*18       |*/ bar2: u32,
    /*1c       |*/ bar3: u32,
    /*20       |*/ bar4: u32,
    /*24       |*/ bar5: u32,
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
    /*40       |*/
}

/// This table is applicable if the Header Type is 01h (PCI-to-PCI bridge)
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PciDeviceType1 {
    /*10       |*/ bar0: u32,
    /*14       |*/ bar1: u32,
    /*18       |*/ primary_bus_number: u8,
    /*19       |*/ secondary_bus_number: u8,
    /*1a       |*/ subordinate_bus_number: u8,
    /*1b       |*/ secondary_latency_timer: u8,
    /*1c       |*/ io_base: u8,
    /*1d       |*/ io_limit: u8,
    /*1e       |*/ secondary_status: u16,
    /*20       |*/ memory_base: u16,
    /*22       |*/ memory_limit: u16,
    /*24       |*/ prefetchable_memory_base: u16,
    /*26       |*/ prefetchable_memory_limit: u16,
    /*28       |*/ prefetchable_base_upper_32_bits: u32,
    /*2c       |*/ prefetchable_limit_upper_32_bits: u32,
    /*30       |*/ io_base_upper_16_bits: u16,
    /*32       |*/ io_limit_upper_16_bits: u16,
    /*34       |*/ capability_pointer: u8,
    /*35       |*/ reserved: [u8; 3],
    /*38       |*/ expansion_rom_base_address: u32,
    /*3c       |*/ interrupt_line: u8,
    /*3d       |*/ interrupt_pin: u8,
    /*3e       |*/ bridge_control: u16,
    /*40       |*/
}

/// This table is applicable if the Header Type is 02h (PCI-to-CardBus bridge)
/// TODO Be carefull, there is a mistake in documentation. offset exceed 0x40
#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct PciDeviceType2 {
    /*10       |*/ card_bus_socket_exca_base_address: u32,
    /*14       |*/ offset_of_capabilities_list: u8,
    /*15       |*/ reserved: u8,
    /*16       |*/ secondary_status: u16,
    /*18       |*/ pci_bus_number: u8,
    /*19       |*/ card_bus_bus_number: u8,
    /*1a       |*/ subordinate_bus_number: u8,
    /*1b       |*/ card_bus_latency_timer: u8,
    /*1c       |*/ memory_base_address_0: u32,
    /*20       |*/ memory_limit_0: u32,
    /*24       |*/ memory_base_address_1: u32,
    /*28       |*/ memory_limit_1: u32,
    /*2c       |*/ io_base_address_0: u32,
    /*30       |*/ io_limit_0: u32,
    /*34       |*/ io_base_address_1: u32,
    /*38       |*/ io_limit_1: u32,
    /*3c       |*/ interrupt_line: u8,
    /*3d       |*/ interrupt_pin: u8,
    /*3e       |*/ bridge_control: u16,
    /*40       |*/
}
// Non-coherant documentation
/*
/*40       |*/ subsystem_device_id: u16,
/*42       |*/ subsystem_vendor_id: u16,
/*44       |*/ b16_pc_card_legacy_mode_base_address: u32,
 */

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PciType0 {
    /*0        |*/ pub vendor_id: u16,
    /*2        |*/ pub device_id: u16,
    /*4        |*/ pub command: u16,
    /*6        |*/ pub status: u16,
    /*8        |*/ pub revision_id: u8,
    /*9        |*/ pub prog_if: u8,
    /*a        |*/ pub sub_class: u8,
    /*b        |*/ pub class_code: u8,
    /*c        |*/ pub cache_line_size: u8,
    /*d        |*/ pub latency_timer: u8,
    /*e        |*/ pub header_type: u8,
    /*f        |*/ pub bist: u8,
    /*10       |*/ pub bar0: u32,
    /*14       |*/ pub bar1: u32,
    /*18       |*/ pub bar2: u32,
    /*1c       |*/ pub bar3: u32,
    /*20       |*/ pub bar4: u32,
    /*24       |*/ pub bar5: u32,
    /*28       |*/ pub cardbus_cis_pointer: u32,
    /*2c       |*/ pub subsystem_vendor_id: u16,
    /*2e       |*/ pub subsystem_id: u16,
    /*30       |*/ pub expansion_rom_base_address: u32,
    /*34       |*/ pub capabilities_pointer: u8,
    /*35       |*/ pub reserved: [u8; 7],
    /*3c       |*/ pub interrupt_line: u8,
    /*3d       |*/ pub interrupt_pin: u8,
    /*3e       |*/ pub min_grant: u8,
    /*3f       |*/ pub max_latency: u8,
}

// List of PCI commands
bitflags! {
    pub struct PciCommand: u16 {
        const INTERRUPT_DISABLE = 1 << 10; // If set to 1 the assertion of the devices INTx# signal is disabled; otherwise, assertion of the signal is enabled.
        const FAST_BACK_TO_BACK_ENABLE = 1 << 9; // If set to 1 indicates a device is allowed to generate fast back-to-back transactions; otherwise, fast back-to-back transactions are only allowed to the same agent.
        const SERR_ENABLE = 1 << 8; // If set to 1 the SERR# driver is enabled; otherwise, the driver is disabled.
        const BIT_7 = 1 << 7; // As of revision 3.0 of the PCI local bus specification this bit is hardwired to 0. In earlier versions of the specification this bit was used by devices and may have been hardwired to 0, 1, or implemented as a read/write bit.
        const PARITY_ERROR_RESPONDE = 1 << 6; // If set to 1 the device will take its normal action when a parity error is detected; otherwise, when an error is detected, the device will set bit 15 of the Status register (Detected Parity Error Status Bit), but will not assert the PERR# (Parity Error) pin and will continue operation as normal.
        const VGA_PALETTE_SNOOP = 1 << 5; // If set to 1 the device does not respond to palette register writes and will snoop the data; otherwise, the device will trate palette write accesses like all other accesses.
        const MEMORY_WRITE_AND_INVALIDABLE_ENABLE = 1 << 4; // If set to 1 the device can generate the Memory Write and Invalidate command; otherwise, the Memory Write command must be used.
        const SPECIAL_CYCLES = 1 << 3; // If set to 1 the device can monitor Special Cycle operations; otherwise, the device will ignore them.
        const BUS_MASTER = 1 << 2; // If set to 1 the device can behave as a bus master; otherwise, the device can not generate PCI accesses.
        const MEMORY_SPACE = 1 << 1;  // If set to 1 the device can respond to Memory Space accesses; otherwise, the device's response is disabled.
        const IO_SPACE = 1 << 0;  // If set to 1 the device can respond to I/O Space accesses; otherwise, the device's response is disabled.
    }
}

// List of PCI status
bitflags! {
    pub struct PciStatus: u16 {
        const INTERRUPT_STATUS = 1 << 3; // Represents the state of the device's INTx# signal. If set to 1 and bit 10 of the Command register (Interrupt Disable bit) is set to 0 the signal will be asserted; otherwise, the signal will be ignored.
        const CAPABILITIES_LIST = 1 << 4; // If set to 1 the device implements the pointer for a New Capabilities Linked list at offset 0x34; otherwise, the linked list is not available
        const MHZ_66_CAPABLE = 1 << 5; // If set to 1 the device is capable of running at 66 MHz; otherwise, the device runs at 33 MHz.
        const FAST_BACK_TO_BACK_CAPABLE = 1 << 7; // If set to 1 the device can accept fast back-to-back transactions that are not from the same agent; otherwise, transactions can only be accepted from the same agent.
        const MASTER_DATA_PARITY_ERROR = 1 << 8; //  This bit is only set when the following some conditions are met. The bus agent asserted PERR# on a read or observed an assertion of PERR# on a write, the agent setting the bit acted as the bus master for the operation in which the error occurred, and bit 6 of the Command register (Parity Error Response bit) is set to 1.
        const DEVSEL_TIMING = (1 << 9) | (1 << 10); // Read only bits that represent the slowest time that a device will assert DEVSEL# for any bus command except Configuration Space read and writes. Where a value of 0x00 represents fast timing, a value of 0x01 represents medium timing, and a value of 0x02 represents slow timing.
        const SIGNALED_TARGET_ABORT = 1 << 11; // This bit will be set to 1 whenever a target device terminates a transaction with Target-Abort.
        const RECEIVED_TARGET_ABORT = 1 << 12; // This bit will be set to 1, by a master device, whenever its transaction is terminated with Target-Abort.
        const RECEIVED_MASTER_ABORT = 1 << 13; // This bit will be set to 1, by a master device, whenever its transaction (except for Special Cycle transactions) is terminated with Master-Abort.
        const SIGNALED_SYSTEM_ERROR = 1 << 14; // This bit will be set to 1 whenever the device asserts SERR#
        const DETECTED_PARITY_ERROR = 1 << 15; // This bit will be set to 1 whenever the device detects a parity error, even if parity error handling is disabled.
    }
}

/// PCI boilerplate
impl PciType0 {
    /// Apply a command into PCI bus
    pub fn set_command(&self, command: PciCommand, state: bool, pci_location: u32) {
        Pio::<u32>::new(Pci::CONFIG_ADDRESS).write(pci_location + 4);
        let l1 = Pio::<u32>::new(Pci::CONFIG_DATA).read();

        let c = match state {
            true => PciCommand { bits: l1 as u16 } | command,
            false => PciCommand { bits: l1 as u16 } & !command,
        };

        Pio::<u32>::new(Pci::CONFIG_ADDRESS).write(pci_location + 4);
        Pio::<u16>::new(Pci::CONFIG_DATA).write(c.bits);
    }

    /// Get the status of the PCI bus
    pub fn get_status(&self, pci_location: u32) -> PciStatus {
        Pio::<u32>::new(Pci::CONFIG_ADDRESS).write(pci_location + 4);
        let l1 = Pio::<u32>::new(Pci::CONFIG_DATA).read();

        PciStatus {
            bits: (l1 >> 16) as u16,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
enum PciDeviceRegisters {
    PciType0(PciDeviceType0),
    PciType1(PciDeviceType1),
    PciType2(PciDeviceType2),
}

/// Global structure representing a PCI device
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct PciDevice {
    header_l0: PciDeviceHeaderL0,
    header_body: PciDeviceHeaderBody,
    registers: PciDeviceRegisters,
    class: PciDeviceClass,
    address_space: AddressSpace,
}

#[derive(Copy, Clone, Debug)]
struct AddressSpace {
    bus: u8,
    slot: u8,
    function: u8,
}

impl AddressSpace {
    pub fn get_location(&self) -> u32 {
        0x80000000
            + ((self.bus as u32) << 16)
            + ((self.slot as u32) << 11)
            + ((self.function as u32) << 8)
    }
}

/// Custom debug definition for PCI device
impl core::fmt::Display for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let device_type = match self.registers {
            PciDeviceRegisters::PciType0(_) => "Simple and basic device",
            PciDeviceRegisters::PciType1(_) => "Pci to Pci bridge",
            PciDeviceRegisters::PciType2(_) => "Pci to CardBus bridge",
        };
        write!(
            f,
            "{:02X?}:{:02X?}.{:X?} {:?} {:?}",
            self.address_space.bus,
            self.address_space.slot,
            self.address_space.function,
            self.class,
            device_type
        )
    }
}

/// static  allocator for Self::CAPACITY devices max
pub struct CustomPciDeviceAllocator {
    devices_array: [Option<PciDevice>; Self::CAPACITY],
    len: usize,
}

/// Basic implementation of static allocator for PciDevices
impl CustomPciDeviceAllocator {
    const CAPACITY: usize = 256;

    /// Constructor
    pub const fn new() -> Self {
        CustomPciDeviceAllocator {
            devices_array: [None; Self::CAPACITY],
            len: 0,
        }
    }

    /// Push a new device
    pub fn push(&mut self, s: PciDevice) -> core::result::Result<(), ()> {
        if self.len == Self::CAPACITY {
            Err(())
        } else {
            self.devices_array[self.len] = Some(s);
            self.len += 1;
            Ok(())
        }
    }

    /// Allow Ability to iterate
    pub fn iter(&self) -> CustomPciDeviceAllocatorIterator {
        CustomPciDeviceAllocatorIterator {
            parent_reference: self,
            current_iter: 0,
        }
    }
}

/// Iterator structure definition for CustomPciAllocator
pub struct CustomPciDeviceAllocatorIterator<'a> {
    parent_reference: &'a CustomPciDeviceAllocator,
    current_iter: usize,
}

/// Iterator implementation
impl<'a> Iterator for CustomPciDeviceAllocatorIterator<'a> {
    type Item = PciDevice;

    /// Iterator must have at least a next method
    fn next(&mut self) -> Option<PciDevice> {
        if self.current_iter == self.parent_reference.len {
            None
        } else {
            self.current_iter += 1;
            self.parent_reference.devices_array[self.current_iter - 1]
        }
    }
}

impl Pci {
    /// PCI configuration address
    pub const CONFIG_ADDRESS: u16 = 0x0CF8;
    pub const CONFIG_DATA: u16 = 0x0CFC;

    pub const fn new() -> Pci {
        Pci {
            devices_list: CustomPciDeviceAllocator::new(),
        }
    }

    pub fn query_device<T: Copy + core::fmt::Debug>(
        &mut self,
        device_class: PciDeviceClass,
    ) -> Option<(T, u32)> {
        let dev = self.devices_list.iter().find(|d| d.class == device_class)?;
        let location = dev.address_space.get_location();
        let dev = unsafe { transmute_copy::<PciDeviceRaw, T>(&PciDeviceRaw::fill(location)) };
        Some((dev, location))
    }

    /// Output all connected pci devices
    pub fn scan_pci_buses(&mut self) {
        // Simple and Basic brute force scan method is used here !
        for bus in 0..=255 {
            for slot in 0..=31 {
                match self.check_device(bus, slot, 0) {
                    Some(device) => {
                        self.devices_list.push(device).unwrap();
                        // check if is a multi-function device
                        if device.header_body.header_type.get_bit(7) {
                            for function in 1..=7 {
                                match self.check_device(bus, slot, function) {
                                    Some(device) => {
                                        self.devices_list.push(device).unwrap();
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }

    /// List and enumerate all devices
    pub fn list_pci_devices(&mut self) {
        for (i, elem) in self.devices_list.iter().enumerate() {
            println!("{} {}", i, elem);
        }
    }

    /// Bit 31 is the 'enable bit', for configuring cycles, it is necessary
    /// to read the device space through IO port and to configure it !
    /// ---> 0x80_00_00_00 (must be confirmed by Sclolus)
    ///                         |  |      PCI BUS       |  |                  32 bits
    ///                    __________________________________________
    ///                   | |||| |||| |||| |||| |||| |||| |||| |||| |
    ///                     |||| |||| |||| |||| |||| |||| |||| ||||
    /// 0x8000        -> 0x 1000 0000 ---- ---- ---- ---- ---- ----
    /// bus << 16     -> 0x ---- ---- BBBB BBBB ---- ---- ---- ----   -> 256 values   0x00 -> 0xFF
    /// slot << 11    -> 0x ---- ---- ---- ---- BBBB B--- ---- ----   ->  32 values   0x00 -> 0x20
    /// function << 8 -> 0x ---- ---- ---- ---- ---- -BBB ---- ----   ->   7 values   0x00 -> 0x08
    /// register << 2 -> 0x ---- ---- ---- ---- ---- ---- BBBB BB--   ->  64 values   0x00 -> 0x40

    /// Take a device location as argument and check if a device exists here
    /// return PciDevice on success
    fn check_device(&self, bus: u8, slot: u8, function: u8) -> Option<PciDevice> {
        let mut location: u32 = 0x80000000;
        location += (bus as u32) << 16;
        location += (slot as u32) << 11;
        location += (function as u32) << 8;

        let header_l0 = unsafe {
            transmute_copy::<PciDeviceHeaderL0Raw, PciDeviceHeaderL0>(&PciDeviceHeaderL0Raw::fill(
                location,
            ))
        };

        match header_l0.vendor_id {
            0xffff => None,
            _ => {
                location += size_of::<PciDeviceHeaderL0Raw>() as u32;
                let header_body = unsafe {
                    transmute_copy::<PciDeviceHeaderBodyRaw, PciDeviceHeaderBody>(
                        &PciDeviceHeaderBodyRaw::fill(location),
                    )
                };

                location += size_of::<PciDeviceHeaderBodyRaw>() as u32;
                let registers = unsafe {
                    match header_body.header_type & 0x3 {
                        0x1 => PciDeviceRegisters::PciType1(transmute_copy(
                            &PciDeviceRegistersRaw::fill(location),
                        )),
                        0x2 => PciDeviceRegisters::PciType2(transmute_copy(
                            &PciDeviceRegistersRaw::fill(location),
                        )),
                        // Default device is considered like PCI type 0
                        _ => PciDeviceRegisters::PciType0(transmute_copy(
                            &PciDeviceRegistersRaw::fill(location),
                        )),
                    }
                };

                Some(PciDevice {
                    header_l0: header_l0,
                    header_body: header_body,
                    registers: registers,
                    class: get_pci_device(
                        header_body.class_code,
                        header_body.sub_class,
                        header_body.prog_if,
                    ),
                    address_space: AddressSpace {
                        bus,
                        slot,
                        function,
                    },
                })
            }
        }
    }
}

/// What is this fucking device ?
fn get_pci_device(id: u8, subclass: u8, prog_if: u8) -> PciDeviceClass {
    use PciDeviceClass::*;
    match id {
        0x0 => {
            use UnclassifiedSubClass::*;
            Unclassified(match subclass {
                0x0 => NonVgaCompatibleDevice,
                0x1 => VgaCompatibleDevice,
                _ => Unknown,
            })
        }
        0x1 => {
            use MassStorageControllerSubClass::*;
            MassStorageController(match subclass {
                0x0 => ScsiBusController,
                0x1 => {
                    use IdeControllerProgIf::*;
                    IdeController(match prog_if {
                        0x0 => IsaCompatibilityModeOnlyController,
                        0x5 => PciNativeModeOnlyController,
                        0xA => IsaCompatibilityModeController,
                        0xF => PciNativeModeController,
                        0x80 => IsaCompatibilityModeOnlyControllerBusMastered,
                        0x85 => PciNativeModeOnlyControllerBusMastered,
                        0x8A => IsaCompatibilityModeControllerBusMastered,
                        0x8F => PciNativeModeControllerBusMastered,
                        _ => Unknown,
                    })
                }
                0x2 => FloppyDiskController,
                0x3 => IpiBusController,
                0x4 => RaidController,
                0x5 => {
                    use AtaControllerProgIf::*;
                    AtaController(match prog_if {
                        0x20 => SingleDma,
                        0x30 => ChainedDma,
                        _ => Unknown,
                    })
                }
                0x6 => {
                    use SerialAtaProgIf::*;
                    SerialAta(match prog_if {
                        0x0 => VendorSpecificInterface,
                        0x1 => Ahci1,
                        0x2 => SerialStorageBus,
                        _ => Unknown,
                    })
                }
                0x7 => {
                    use SerialAttachedScsiProgIf::*;
                    SerialAttachedScsi(match prog_if {
                        0x0 => Sas,
                        0x1 => SerialStorageBus,
                        _ => Unknown,
                    })
                }
                0x8 => {
                    use NonVolatileMemoryControllerProgIf::*;
                    NonVolatileMemoryController(match prog_if {
                        0x1 => Nvmhci,
                        0x2 => NvmExpress,
                        _ => Unknown,
                    })
                }
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x2 => {
            use NetworkControllerSubClass::*;
            NetworkController(match subclass {
                0x0 => EthernetController,
                0x1 => TokenRingController,
                0x2 => FddiController,
                0x3 => AtmController,
                0x4 => IsdnController,
                0x5 => WorldFipController,
                0x6 => Picmg214MultiComputing,
                0x7 => InfinibandController,
                0x8 => FabricController,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x3 => {
            use DisplayControllerSubClass::*;
            DisplayController(match subclass {
                0x0 => {
                    use VgaCompatibleControllerProgIf::*;
                    VgaCompatibleController(match prog_if {
                        0x0 => VgaController,
                        0x1 => Compatible8514Controller,
                        _ => Unknown,
                    })
                }
                0x1 => XgaController,
                0x2 => Controller3d,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x4 => {
            use MultimediaControllerSubClass::*;
            MultimediaController(match subclass {
                0x0 => MultimediaVideoController,
                0x1 => MultimediaAudioController,
                0x2 => ComputerTelephonyDevice,
                0x3 => AudioDevice,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x5 => {
            use MemoryControllerSubClass::*;
            MemoryController(match subclass {
                0x0 => RamController,
                0x1 => FlashController,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x6 => {
            use BridgeDeviceSubClass::*;
            BridgeDevice(match subclass {
                0x0 => HostBridge,
                0x1 => IsaBridge,
                0x2 => EisaBridge,
                0x3 => McaBridge,
                0x4 => {
                    use PciToPciBridgeProgIf::*;
                    PciToPciBridge(match prog_if {
                        0x0 => NormalDecode,
                        0x1 => SubtractiveDecode,
                        _ => Unknown,
                    })
                }
                0x5 => PcmciaBridge,
                0x6 => NuBusBridge,
                0x7 => CardBusBridge,
                0x8 => {
                    use RaceWayBridgeProgIf::*;
                    RaceWayBridge(match prog_if {
                        0x0 => TransparentMode,
                        0x1 => EndpointMode,
                        _ => Unknown,
                    })
                }
                0x9 => {
                    use PciToPciBridge2ProgIf::*;
                    PciToPciBridge2(match prog_if {
                        0x40 => SemiTransparentPrimaryBusTowardsHostCPU,
                        0x80 => SemiTransparentSecondaryBusTowardsHostCPU,
                        _ => Unknown,
                    })
                }
                0xA => InfiniBandToPciHostBridge,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x7 => {
            use SimpleCommunicationControllerSubClass::*;
            SimpleCommunicationController(match subclass {
                0x0 => {
                    use SerialControllerProgIf::*;
                    SerialController(match prog_if {
                        0x0 => CompatibleGenericXT,
                        0x1 => Compatible16450,
                        0x2 => Compatible16550,
                        0x3 => Compatible16650,
                        0x4 => Compatible16750,
                        0x5 => Compatible16850,
                        0x6 => Compatible16950,
                        _ => Unknown,
                    })
                }
                0x1 => {
                    use ParallelControllerProgIf::*;
                    ParallelController(match prog_if {
                        0x0 => StandardParallelPort,
                        0x1 => BiDirectionalParallelPort,
                        0x2 => ECPv1XCompliantParallelPort,
                        0x3 => IEEE1284Controller,
                        0xFE => IEEE1284TargetDevice,
                        _ => Unknown,
                    })
                }
                0x2 => MultiportSerialController,
                0x3 => {
                    use ModemProgIf::*;
                    Modem(match prog_if {
                        0x0 => GenericModem,
                        0x1 => Hayes16450CompatibleInterface,
                        0x2 => Hayes16550CompatibleInterface,
                        0x3 => Hayes16650CompatibleInterface,
                        0x4 => Hayes16750CompatibleInterface,
                        _ => Unknown,
                    })
                }
                0x4 => Ieee488v1s2GpibController,
                0x5 => SmartCard,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x8 => {
            use BaseSystemPeripheralSubClass::*;
            BaseSystemPeripheral(match subclass {
                0x0 => {
                    use PicProgIf::*;
                    Pic(match prog_if {
                        0x0 => Generic8259Compatible,
                        0x1 => ISACompatible,
                        0x2 => EISACompatible,
                        0x10 => IOAPICInterruptController,
                        0x20 => IOxAPICInterruptController,
                        _ => Unknown,
                    })
                }
                0x1 => {
                    use DmaControllerProgIf::*;
                    DmaController(match prog_if {
                        0x0 => Generic8237Compatible,
                        0x1 => ISACompatible,
                        0x2 => EISACompatible,
                        _ => Unknown,
                    })
                }
                0x2 => {
                    use TimerProgIf::*;
                    Timer(match prog_if {
                        0x0 => Generic8254Compatible,
                        0x1 => ISACompatible,
                        0x2 => EISACompatible,
                        0x3 => HPET,
                        _ => Unknown,
                    })
                }
                0x3 => {
                    use RtcControllerProgIf::*;
                    RtcController(match prog_if {
                        0x0 => GenericRTC,
                        0x1 => ISACompatible,
                        _ => Unknown,
                    })
                }
                0x4 => PciHotPlugController,
                0x5 => SdHostController,
                0x6 => Iommu,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x9 => {
            use InputDeviceControllerSubClass::*;
            InputDeviceController(match subclass {
                0x0 => KeyboardController,
                0x1 => DigitizerPen,
                0x2 => MouseController,
                0x3 => ScannerController,
                0x4 => {
                    use GameportControllerProgIf::*;
                    GameportController(match prog_if {
                        0x0 => Generic,
                        0x10 => Extended,
                        _ => Unknown,
                    })
                }
                0x80 => Other,
                _ => Unknown,
            })
        }
        0xA => {
            use DockingStationSubClass::*;
            DockingStation(match subclass {
                0x0 => Generic,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0xB => {
            use ProcessorSubClass::*;
            Processor(match subclass {
                0x0 => I386,
                0x1 => I486,
                0x2 => Pentium,
                0x10 => Alpha,
                0x20 => PowerPC,
                0x30 => Mips,
                0x40 => CoProcessor,
                _ => Unknown,
            })
        }
        0xC => {
            use SerialBusControllerSubClass::*;
            SerialBusController(match subclass {
                0x0 => {
                    use FireWireIeee1394ControllerProgIf::*;
                    FireWireIeee1394Controller(match prog_if {
                        0x0 => Generic,
                        0x10 => OHCI,
                        _ => Unknown,
                    })
                }
                0x1 => AccessBus,
                0x2 => Ssa,
                0x3 => {
                    use UsbControllerProgIf::*;
                    UsbController(match prog_if {
                        0x0 => UHCIController,
                        0x10 => OHCIController,
                        0x20 => EHCIUsb2Controller,
                        0x30 => XHCIUsb3Controller,
                        0x80 => Unspecified,
                        0xFE => USBDeviceNotAHostControler,
                        _ => Unknown,
                    })
                }
                0x4 => FibreChannel,
                0x5 => SmBus,
                0x6 => InfiniBand,
                0x7 => {
                    use IpmiInterfaceProgIf::*;
                    IpmiInterface(match prog_if {
                        0x0 => SMIC,
                        0x1 => KeyboardControllerStyle,
                        0x2 => BlockTransfer,
                        _ => Unknown,
                    })
                }
                0x8 => SercosInterfaceIec61491,
                0x9 => CANbus,
                _ => Unknown,
            })
        }
        0xD => {
            use WirelessControllerSubClass::*;
            WirelessController(match subclass {
                0x0 => IrdaCompatibleController,
                0x1 => ConsumerIrController,
                0x10 => RfController,
                0x11 => BluetoothController,
                0x12 => BroadbandController,
                0x20 => EthernetController802v1a,
                0x21 => EthernetController802v1b,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0xE => {
            use IntelligentControllerSubClass::*;
            IntelligentController(match subclass {
                0x0 => I20,
                _ => Unknown,
            })
        }
        0xF => {
            use SatelliteCommunicationControllerSubClass::*;
            SatelliteCommunicationController(match subclass {
                0x1 => SatelliteTVController,
                0x2 => SatelliteAudioController,
                0x3 => SatelliteVoiceController,
                0x4 => SatelliteDataController,
                _ => Unknown,
            })
        }
        0x10 => {
            use EncryptionControllerSubClass::*;
            EncryptionController(match subclass {
                0x0 => NetworkAndComputingEncrpytionDecryption,
                0x10 => EntertainmentEncryptionDecryption,
                0x80 => OtherEncryptionDecryption,
                _ => Unknown,
            })
        }
        0x11 => {
            use SignalProcessingControllerSubClass::*;
            SignalProcessingController(match subclass {
                0x0 => DpioModules,
                0x1 => PerformanceCounters,
                0x10 => CommunicationSynchronizer,
                0x20 => SignalProcessingManagement,
                0x80 => Other,
                _ => Unknown,
            })
        }
        0x12 => ProcessingAccelerator,
        0x13 => NonEssentialInstrumentation,
        0x14 => Reserved0x3f,
        0x40 => CoProcessor,
        0x41 => Reserved0xfe,
        0xFF => UnassignedClassVendorSpecific,
        _ => Unknown,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PciDeviceClass {
    Unclassified(UnclassifiedSubClass),
    MassStorageController(MassStorageControllerSubClass),
    NetworkController(NetworkControllerSubClass),
    DisplayController(DisplayControllerSubClass),
    MultimediaController(MultimediaControllerSubClass),
    MemoryController(MemoryControllerSubClass),
    BridgeDevice(BridgeDeviceSubClass),
    SimpleCommunicationController(SimpleCommunicationControllerSubClass),
    BaseSystemPeripheral(BaseSystemPeripheralSubClass),
    InputDeviceController(InputDeviceControllerSubClass),
    DockingStation(DockingStationSubClass),
    Processor(ProcessorSubClass),
    SerialBusController(SerialBusControllerSubClass),
    WirelessController(WirelessControllerSubClass),
    IntelligentController(IntelligentControllerSubClass),
    SatelliteCommunicationController(SatelliteCommunicationControllerSubClass),
    EncryptionController(EncryptionControllerSubClass),
    SignalProcessingController(SignalProcessingControllerSubClass),
    ProcessingAccelerator,
    NonEssentialInstrumentation,
    Reserved0x3f,
    CoProcessor,
    Reserved0xfe,
    UnassignedClassVendorSpecific,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnclassifiedSubClass {
    NonVgaCompatibleDevice,
    VgaCompatibleDevice,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MassStorageControllerSubClass {
    ScsiBusController,
    IdeController(IdeControllerProgIf),
    FloppyDiskController,
    IpiBusController,
    RaidController,
    AtaController(AtaControllerProgIf),
    SerialAta(SerialAtaProgIf),
    SerialAttachedScsi(SerialAttachedScsiProgIf),
    NonVolatileMemoryController(NonVolatileMemoryControllerProgIf),
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IdeControllerProgIf {
    IsaCompatibilityModeOnlyController,
    PciNativeModeOnlyController,
    IsaCompatibilityModeController,
    PciNativeModeController,
    IsaCompatibilityModeOnlyControllerBusMastered,
    PciNativeModeOnlyControllerBusMastered,
    IsaCompatibilityModeControllerBusMastered,
    PciNativeModeControllerBusMastered,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtaControllerProgIf {
    SingleDma,
    ChainedDma,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialAtaProgIf {
    VendorSpecificInterface,
    Ahci1,
    SerialStorageBus,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialAttachedScsiProgIf {
    Sas,
    SerialStorageBus,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NonVolatileMemoryControllerProgIf {
    Nvmhci,
    NvmExpress,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NetworkControllerSubClass {
    EthernetController,
    TokenRingController,
    FddiController,
    AtmController,
    IsdnController,
    WorldFipController,
    Picmg214MultiComputing,
    InfinibandController,
    FabricController,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DisplayControllerSubClass {
    VgaCompatibleController(VgaCompatibleControllerProgIf),
    XgaController,
    Controller3d,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VgaCompatibleControllerProgIf {
    VgaController,
    Compatible8514Controller,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MultimediaControllerSubClass {
    MultimediaVideoController,
    MultimediaAudioController,
    ComputerTelephonyDevice,
    AudioDevice,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryControllerSubClass {
    RamController,
    FlashController,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BridgeDeviceSubClass {
    HostBridge,
    IsaBridge,
    EisaBridge,
    McaBridge,
    PciToPciBridge(PciToPciBridgeProgIf),
    PcmciaBridge,
    NuBusBridge,
    CardBusBridge,
    RaceWayBridge(RaceWayBridgeProgIf),
    PciToPciBridge2(PciToPciBridge2ProgIf),
    InfiniBandToPciHostBridge,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PciToPciBridgeProgIf {
    NormalDecode,
    SubtractiveDecode,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RaceWayBridgeProgIf {
    TransparentMode,
    EndpointMode,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PciToPciBridge2ProgIf {
    SemiTransparentPrimaryBusTowardsHostCPU,
    SemiTransparentSecondaryBusTowardsHostCPU,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SimpleCommunicationControllerSubClass {
    SerialController(SerialControllerProgIf),
    ParallelController(ParallelControllerProgIf),
    MultiportSerialController,
    Modem(ModemProgIf),
    Ieee488v1s2GpibController,
    SmartCard,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialControllerProgIf {
    CompatibleGenericXT,
    Compatible16450,
    Compatible16550,
    Compatible16650,
    Compatible16750,
    Compatible16850,
    Compatible16950,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParallelControllerProgIf {
    StandardParallelPort,
    BiDirectionalParallelPort,
    ECPv1XCompliantParallelPort,
    IEEE1284Controller,
    IEEE1284TargetDevice,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ModemProgIf {
    GenericModem,
    Hayes16450CompatibleInterface,
    Hayes16550CompatibleInterface,
    Hayes16650CompatibleInterface,
    Hayes16750CompatibleInterface,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BaseSystemPeripheralSubClass {
    Pic(PicProgIf),
    DmaController(DmaControllerProgIf),
    Timer(TimerProgIf),
    RtcController(RtcControllerProgIf),
    PciHotPlugController,
    SdHostController,
    Iommu,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PicProgIf {
    Generic8259Compatible,
    ISACompatible,
    EISACompatible,
    IOAPICInterruptController,
    IOxAPICInterruptController,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DmaControllerProgIf {
    Generic8237Compatible,
    ISACompatible,
    EISACompatible,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimerProgIf {
    Generic8254Compatible,
    ISACompatible,
    EISACompatible,
    HPET,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RtcControllerProgIf {
    GenericRTC,
    ISACompatible,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputDeviceControllerSubClass {
    KeyboardController,
    DigitizerPen,
    MouseController,
    ScannerController,
    GameportController(GameportControllerProgIf),
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameportControllerProgIf {
    Generic,
    Extended,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DockingStationSubClass {
    Generic,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProcessorSubClass {
    I386,
    I486,
    Pentium,
    Alpha,
    PowerPC,
    Mips,
    CoProcessor,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SerialBusControllerSubClass {
    FireWireIeee1394Controller(FireWireIeee1394ControllerProgIf),
    AccessBus,
    Ssa,
    UsbController(UsbControllerProgIf),
    FibreChannel,
    SmBus,
    InfiniBand,
    IpmiInterface(IpmiInterfaceProgIf),
    SercosInterfaceIec61491,
    CANbus,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FireWireIeee1394ControllerProgIf {
    Generic,
    OHCI,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UsbControllerProgIf {
    UHCIController,
    OHCIController,
    EHCIUsb2Controller,
    XHCIUsb3Controller,
    Unspecified,
    USBDeviceNotAHostControler,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IpmiInterfaceProgIf {
    SMIC,
    KeyboardControllerStyle,
    BlockTransfer,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WirelessControllerSubClass {
    IrdaCompatibleController,
    ConsumerIrController,
    RfController,
    BluetoothController,
    BroadbandController,
    EthernetController802v1a,
    EthernetController802v1b,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntelligentControllerSubClass {
    I20,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SatelliteCommunicationControllerSubClass {
    SatelliteTVController,
    SatelliteAudioController,
    SatelliteVoiceController,
    SatelliteDataController,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EncryptionControllerSubClass {
    NetworkAndComputingEncrpytionDecryption,
    EntertainmentEncryptionDecryption,
    OtherEncryptionDecryption,
    Unknown,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SignalProcessingControllerSubClass {
    DpioModules,
    PerformanceCounters,
    CommunicationSynchronizer,
    SignalProcessingManagement,
    Other,
    Unknown,
}
