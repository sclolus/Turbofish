//! See [PCI](https://wiki.osdev.org/PCI)
use crate::io::{Io, Pio};

use bit_field::BitField;

pub struct Pci {}

pub static mut PCI: Pci = Pci::new();

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

/// Rust abstract of First line of PCI header
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceHeaderL0Raw {
        l0: u32,
    }
);

/// Rust Abstract of next third lines of PCI header
fill_struct_with_io!(
    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    struct PciDeviceHeaderBodyRaw {
        l1: u32,
        l2: u32,
        l3: u32,
    }
);

/// Rust Abstract of Pci Registers (body)
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
    /*3e       |*/
    bridge_control: u16,
    /*40       |*/

        // Non-coherant documentation
    /*  /*40       |*/ subsystem_device_id: u16,
        /*42       |*/ subsystem_vendor_id: u16,
        /*44       |*/ b16_pc_card_legacy_mode_base_address: u32, */
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
enum PciDeviceRegisters {
    PciType0(PciDeviceType0),
    PciType1(PciDeviceType1),
    PciType2(PciDeviceType2),
}

/// Global structure representing a PCI device
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct PciDevice {
    header_l0: PciDeviceHeaderL0,
    header_body: PciDeviceHeaderBody,
    registers: PciDeviceRegisters,
    class: PciDeviceClass,
    bus: u8,
    slot: u8,
    function: u8,
}

/// Custom debug definition for PCI device
impl core::fmt::Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let device_type = match self.registers {
            PciDeviceRegisters::PciType0(_) => "Simple and basic device",
            PciDeviceRegisters::PciType1(_) => "Pci to Pci bridge",
            PciDeviceRegisters::PciType2(_) => "Pci to CardBus bridge",
        };
        write!(f, "{:02X?}:{:02X?}.{:X?} {:?} {:?}", self.bus, self.slot, self.function, self.class, device_type)
    }
}

impl Pci {
    /// PCI configuration address
    const CONFIG_ADDRESS: u16 = 0x0CF8;
    const CONFIG_DATA: u16 = 0x0CFC;

    pub const fn new() -> Pci {
        Pci {}
    }

    /// Output all connected pci devices
    pub fn scan_pci_buses(&self) {
        println!("scanning PCI buses ...");

        // Simple and Basic brute force scan method is used here !
        for bus in 0..=255 {
            for slot in 0..=31 {
                match self.check_device(bus, slot, 0) {
                    Some(device) => {
                        println!("{:?}", device);
                        // check if is a multi-function device
                        if device.header_body.header_type.get_bit(7) {
                            println!("Multifunction device detected !");
                            for function in 1..=7 {
                                match self.check_device(bus, slot, function) {
                                    Some(device) => println!("{:?}", device),
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

    ///                         |  |      PCI BUS       |  |                  32 bits
    ///                    __________________________________________
    ///                   | |||| |||| |||| |||| |||| |||| |||| |||| |
    ///                     |||| |||| |||| |||| |||| |||| |||| ||||
    /// 0x8000        -> 0x 1000 0000 ---- ---- ---- ---- ---- ----
    /// bus << 16     -> 0x ---- ---- BBBB BBBB ---- ---- ---- ----   -> 256 values   0x00 -> 0xFF
    /// slot << 11    -> 0x ---- ---- ---- ---- BBBB B--- ---- ----   ->  32 values   0x00 -> 0x20
    /// function << 8 -> 0x ---- ---- ---- ---- ---- -BBB ---- ----   ->   7 values   0x00 -> 0x08
    /// register << 2 -> 0x ---- ---- ---- ---- ---- ---- BBBB BB--   ->  64 values   0x00 -> 0x40

    /// Take a device location as argument and check is device exists here
    /// return PciDevice on success
    fn check_device(&self, bus: u8, slot: u8, function: u8) -> Option<PciDevice> {
        use core::mem::{size_of, transmute_copy};

        let mut location: u32 = 0x80000000 + ((bus as u32) << 16) + ((slot as u32) << 11) + ((function as u32) << 8);

        let header_l0 =
            unsafe { transmute_copy::<PciDeviceHeaderL0Raw, PciDeviceHeaderL0>(&PciDeviceHeaderL0Raw::fill(location)) };

        match header_l0.vendor_id {
            0xffff => None,
            _ => {
                location += size_of::<PciDeviceHeaderL0Raw>() as u32;
                let header_body = unsafe {
                    transmute_copy::<PciDeviceHeaderBodyRaw, PciDeviceHeaderBody>(&PciDeviceHeaderBodyRaw::fill(
                        location,
                    ))
                };

                location += size_of::<PciDeviceHeaderBodyRaw>() as u32;
                let registers = unsafe {
                    match header_body.header_type & 0x3 {
                        0x1 => PciDeviceRegisters::PciType1(transmute_copy(&PciDeviceRegistersRaw::fill(location))),
                        0x2 => PciDeviceRegisters::PciType2(transmute_copy(&PciDeviceRegistersRaw::fill(location))),
                        // Default device is considered like PCI type 0
                        _ => PciDeviceRegisters::PciType0(transmute_copy(&PciDeviceRegistersRaw::fill(location))),
                    }
                };

                Some(PciDevice {
                    header_l0: header_l0,
                    header_body: header_body,
                    registers: registers,
                    class: get_pci_device(header_body.class_code, header_body.sub_class, header_body.prog_if),
                    bus: bus,
                    slot: slot,
                    function: function,
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum UnclassifiedSubClass {
    NonVgaCompatibleDevice,
    VgaCompatibleDevice,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum AtaControllerProgIf {
    SingleDma,
    ChainedDma,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum SerialAtaProgIf {
    VendorSpecificInterface,
    Ahci1,
    SerialStorageBus,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum SerialAttachedScsiProgIf {
    Sas,
    SerialStorageBus,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum NonVolatileMemoryControllerProgIf {
    Nvmhci,
    NvmExpress,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum DisplayControllerSubClass {
    VgaCompatibleController(VgaCompatibleControllerProgIf),
    XgaController,
    Controller3d,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum VgaCompatibleControllerProgIf {
    VgaController,
    Compatible8514Controller,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum MultimediaControllerSubClass {
    MultimediaVideoController,
    MultimediaAudioController,
    ComputerTelephonyDevice,
    AudioDevice,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum MemoryControllerSubClass {
    RamController,
    FlashController,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum PciToPciBridgeProgIf {
    NormalDecode,
    SubtractiveDecode,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum RaceWayBridgeProgIf {
    TransparentMode,
    EndpointMode,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum PciToPciBridge2ProgIf {
    SemiTransparentPrimaryBusTowardsHostCPU,
    SemiTransparentSecondaryBusTowardsHostCPU,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum ParallelControllerProgIf {
    StandardParallelPort,
    BiDirectionalParallelPort,
    ECPv1XCompliantParallelPort,
    IEEE1284Controller,
    IEEE1284TargetDevice,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum ModemProgIf {
    GenericModem,
    Hayes16450CompatibleInterface,
    Hayes16550CompatibleInterface,
    Hayes16650CompatibleInterface,
    Hayes16750CompatibleInterface,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum PicProgIf {
    Generic8259Compatible,
    ISACompatible,
    EISACompatible,
    IOAPICInterruptController,
    IOxAPICInterruptController,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum DmaControllerProgIf {
    Generic8237Compatible,
    ISACompatible,
    EISACompatible,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum TimerProgIf {
    Generic8254Compatible,
    ISACompatible,
    EISACompatible,
    HPET,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum RtcControllerProgIf {
    GenericRTC,
    ISACompatible,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum InputDeviceControllerSubClass {
    KeyboardController,
    DigitizerPen,
    MouseController,
    ScannerController,
    GameportController(GameportControllerProgIf),
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum GameportControllerProgIf {
    Generic,
    Extended,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum DockingStationSubClass {
    Generic,
    Other,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum FireWireIeee1394ControllerProgIf {
    Generic,
    OHCI,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum UsbControllerProgIf {
    UHCIController,
    OHCIController,
    EHCIUsb2Controller,
    XHCIUsb3Controller,
    Unspecified,
    USBDeviceNotAHostControler,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum IpmiInterfaceProgIf {
    SMIC,
    KeyboardControllerStyle,
    BlockTransfer,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum IntelligentControllerSubClass {
    I20,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum SatelliteCommunicationControllerSubClass {
    SatelliteTVController,
    SatelliteAudioController,
    SatelliteVoiceController,
    SatelliteDataController,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum EncryptionControllerSubClass {
    NetworkAndComputingEncrpytionDecryption,
    EntertainmentEncryptionDecryption,
    OtherEncryptionDecryption,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum SignalProcessingControllerSubClass {
    DpioModules,
    PerformanceCounters,
    CommunicationSynchronizer,
    SignalProcessingManagement,
    Other,
    Unknown,
}
