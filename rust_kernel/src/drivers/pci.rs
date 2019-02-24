struct Pci {}

impl Pci {}

pub fn get_conponent(id: u8, subclass: u8, prog_if: u8) -> Class {
    use Class::*;
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
pub enum Class {
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
