//! This files contains the code related to the Initialization Control Words of the PIC 8269 chip.
//! See [PIC](https://wiki.osdev.org/PIC).
//! See https://pdos.csail.mit.edu/6.828/2012/readings/hardware/8259A.pdf (Intel specification).

use super::{BufferingMode, PICKind, TriggeringMode};
use bit_field::BitField;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW1 {
    pub byte: u8,
}

impl ICW1 {
    pub fn new() -> Self {
        // Bit 4 must be set.
        Self { byte: 0b10000 }
    }

    /// If set, this flag indicates that the initialization procedure will require
    /// a fourth Initialization Control Word.
    pub fn set_icw4_needed(mut self, value: bool) -> Self {
        self.byte.set_bit(0, value);
        self
    }

    pub fn get_icw4_needed(self) -> bool {
        self.byte.get_bit(0)
    }

    /// If `value` is true, then single mode is activated.
    /// Otherwise, cascading mode is activated.
    pub fn set_single_mode(mut self, value: bool) -> Self {
        self.byte.set_bit(1, value);
        self
    }

    /// Sets the call address interval to 4 if `value` is true.
    /// Sets it to 8 otherwise.
    pub fn set_call_address_interval(mut self, value: bool) -> Self {
        self.byte.set_bit(2, value);
        self
    }

    /// Sets the triggering mode of the PIC to `mode`.
    pub fn set_triggering_mode(mut self, mode: TriggeringMode) -> Self {
        self.byte.set_bit(3, mode == TriggeringMode::Level);
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW2 {
    pub byte: u8,
}

impl ICW2 {
    pub fn new() -> Self {
        Self { byte: 0 }
    }

    pub fn set_interrupt_vector(mut self, vector: u8) -> Self {
        // The 3 lowest bits are not used in 8086/8088-mode.

        // vector >>= 3;
        // self.byte.set_bits(3..=7, vector);
        self.byte = vector;
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW3 {
    kind: PICKind,
    pub byte: u8,
    cascaded_lines: u8,
}

impl ICW3 {
    pub fn new(kind: PICKind) -> Self {
        Self {
            kind,
            byte: 0,
            cascaded_lines: 0,
        }
    }

    pub fn set_cascaded_line(mut self, line: usize, value: bool) -> Self {
        if self.kind == PICKind::Slave {
            panic!("Tried to set some irq line as cascade for a slave PIC");
        } else if line > 7 {
            panic!("Invalid irq line number provided");
        }

        self.cascaded_lines += 1;
        self.byte.set_bit(line, value);
        self
    }

    pub fn set_slave_identity(mut self, id: u8) -> Self {
        if self.kind == PICKind::Master {
            panic!("Tried to set slave identity for a Master PIC");
        } else if id > 7 {
            panic!("Invalid slave id: {}", id);
        }

        self.byte = id;
        self
    }

    pub fn cascaded_lines(&self) -> usize {
        if self.kind == PICKind::Slave {
            panic!("Only a PIC master can have cascaded lines");
        }
        self.cascaded_lines as usize
    }

    pub fn pic_kind(&self) -> PICKind {
        self.kind
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW4 {
    pub byte: u8,
}

impl ICW4 {
    pub fn new() -> Self {
        Self {
            // We only support the 8086/8088-mode
            byte: 0b1,
        }
    }

    pub fn set_automatic_eio(mut self, value: bool) -> Self {
        self.byte.set_bit(1, value);
        self
    }

    pub fn set_buffering_mode(mut self, mode: BufferingMode) -> Self {
        use BufferingMode::*;

        let value = match mode {
            NotBuffered => 0b00,
            SlaveBuffered => 0b10,
            MasterBuffered => 0b11,
        };

        self.byte.set_bits(2..=3, value);
        self
    }

    pub fn set_special_fully_nested_mode(mut self, value: bool) -> Self {
        self.byte.set_bit(4, value);
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct ICWs {
    pub icw1: Option<ICW1>,
    pub icw2: Option<ICW2>,
    pub icw3: Option<ICW3>,
    pub icw4: Option<ICW4>,
}

impl ICWs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_icw1(mut self, icw: ICW1) -> Self {
        self.icw1 = Some(icw);
        self
    }

    pub fn push_icw2(mut self, icw: ICW2) -> Self {
        self.icw2 = Some(icw);
        self
    }

    pub fn push_icw3(mut self, icw: ICW3) -> Self {
        self.icw3 = Some(icw);
        self
    }

    pub fn push_icw4(mut self, icw: ICW4) -> Self {
        if !self.icw4_needed() {
            panic!("Icw4_needed flag was not set in icw1");
        }
        self.icw4 = Some(icw);
        self
    }

    fn icw4_needed(&self) -> bool {
        self.icw1.expect("Icw1 was not provided").get_icw4_needed()
    }

    pub fn pic_kind(&self) -> PICKind {
        self.icw3.expect("Icw3 was not provided").pic_kind()
    }

    pub fn is_complete(&self) -> bool {
        self.icw1.is_some()
            && self.icw2.is_some()
            && self.icw3.is_some()
            && (!self.icw4_needed() || self.icw4.is_some())
    }

    pub fn cascaded_lines(&self) -> usize {
        assert_eq!(self.pic_kind(), PICKind::Master);
        self.icw3.unwrap().cascaded_lines()
    }
}
