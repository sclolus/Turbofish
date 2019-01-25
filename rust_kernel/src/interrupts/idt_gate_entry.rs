/// See https://wiki.osdev.org/IDT
use bit_field::BitField;
use core::convert::{From, Into};

pub type InterruptHandler = extern "C" fn() -> !;

#[derive(Debug, Copy, Clone)]
pub enum GateType {
    TaskGate32,
    InterruptGate32,
    TrapGate32,
    InterruptGate16,
    TrapGate16,
    UnknownGate,
}

impl From<u8> for GateType {
    fn from(byte: u8) -> Self {
        use GateType::*;

        match byte {
            0x5 => TaskGate32,
            0x6 => InterruptGate16,
            0x7 => TrapGate16,
            0xE => InterruptGate32,
            0xF => TrapGate32,
            _ => UnknownGate,
        }
    }
}

impl Into<u8> for GateType {
    fn into(self) -> u8 {
        use GateType::*;

        match self {
            TaskGate32 => 0x5,
            InterruptGate16 => 0x6,
            TrapGate16 => 0x7,
            InterruptGate32 => 0xE,
            TrapGate32 => 0xF,
            _ => 0xFF, // this really should never happen.
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct IdtGateEntry {
    /// offset bits 0..15. the low part of the address
    pub offset_1: u16,

    /// a code segment selector in GDT or LDT
    pub selector: u16,

    /// unused, set to 0
    pub _zero: u8,

    /// The type attr is layout in this way.
    ///   7                           0
    /// +---+---+---+---+---+---+---+---+
    /// | P |  DPL  | S |    GateType   |
    /// +---+---+---+---+---+---+---+---+

    /// P        	Present	Set to 0 for unused interrupts.
    /// DPL          Descriptor Privilege Level	Gate call protection.
    ///              Specifies which privilege Level the calling Descriptor minimum
    ///              should have.
    ///              So hardware and CPU interrupts can be protected from
    ///              being called out of userspace.
    /// S            Storage Segment	Set to 0 for interrupt and trap gates
    /// Gate Type 	Possible IDT gate types :
    ///              0b0101	0x5	5	80386 32 bit task gate
    ///              0b0110	0x6	6	80286 16-bit interrupt gate
    ///              0b0111	0x7	7	80286 16-bit trap gate
    ///              0b1110	0xE	14	80386 32-bit interrupt gate
    ///              0b1111	0xF	15	80386 32-bit trap gate
    /// type and attributes,
    pub type_attr: u8,

    /// offset bits 16..31
    pub offset_2: u16,
}

impl IdtGateEntry {
    fn minimal() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn new() -> Self {
        let mut new = Self::minimal();

        new.set_present(true);
        new
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.type_attr.set_bit(7, present);
        self
    }

    pub fn set_storage_segment(&mut self, storage: bool) -> &mut Self {
        self.type_attr.set_bit(4, storage);
        self
    }

    pub fn set_gate_type(&mut self, gate_type: GateType) -> &mut Self {
        self.type_attr.set_bits(0..4, gate_type.into());
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u8) -> &mut Self {
        self.type_attr.set_bits(4..6, dpl);
        self
    }

    pub fn set_selector(&mut self, selector: u16) -> &mut Self {
        self.selector = selector;
        self
    }

    pub fn set_handler(&mut self, handler: u32) -> &mut Self {
        self.offset_1 = handler as u16;
        self.offset_2 = ((handler as usize) >> 16) as u16;
        self
    }
}
