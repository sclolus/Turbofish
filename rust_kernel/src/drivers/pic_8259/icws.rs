#![deny(missing_docs)]
//! This files contains the code related to the Initialization Control Words of the PIC 8269 chip.
//! See [PIC](https://wiki.osdev.org/PIC).
//! See https://pdos.csail.mit.edu/6.828/2012/readings/hardware/8259A.pdf (Intel specification).
//!
//! The ICWs (Initialization Control Words) are used to initialize a specific 8259 chip.
//! The procedure is as follows:
//! The ICW1, the first ICW is issued on the command port of the PIC.
//! This starts the initialization procedure, depending of the ICW1,
//! the whole procedure will take 3 to 4 ICWs to complete.
//! This is indicated by the icw4_needed flag of the ICW1.
//! The rest of the ICWs are issued on the data port of the PIC.
//! Once initialized, the chip is ready to issue Interrupts Requests to the CPU.
//! However, it is still possible to modify the behavior of the said chip by issuing
//! to it Operation Control Words (OCWs). Please refer to the ocws module.

use super::{BufferingMode, PICKind, TriggeringMode};
use bit_field::BitField;

/// The grouped Initialization Control Words for a 8259 chip.
/// This data structure is used to initialize a specific 8259 chip.
/// More precisely, the PicConfiguration structure contains 1 or more of those,
/// one per 8259 chips being configured.
///
/// This data structure asserts multiple conditions so that the configuration
/// of a 8259 chip is not malformed from the very beginning.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct ICWs {
    pub icw1: Option<ICW1>,
    pub icw2: Option<ICW2>,
    pub icw3: Option<ICW3>,
    pub icw4: Option<ICW4>,
}

impl ICWs {
    /// Creates a fresh unsetup aggregation of Initialization Control Words.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the first ICW to the data structure.
    pub fn push_icw1(mut self, icw: ICW1) -> Self {
        self.icw1 = Some(icw);
        self
    }

    /// Adds the second ICW to the data structure.
    pub fn push_icw2(mut self, icw: ICW2) -> Self {
        self.icw2 = Some(icw);
        self
    }

    /// Adds the third ICW to the data structure.
    pub fn push_icw3(mut self, icw: ICW3) -> Self {
        self.icw3 = Some(icw);
        self
    }

    /// Adds the fourth (and optional) ICW to the data structure.
    ///
    /// Panic:
    /// Will panic if the ICW1 does not have the icw4_needed flag set.
    pub fn push_icw4(mut self, icw: ICW4) -> Self {
        if !self.icw4_needed() {
            panic!("Icw4_needed flag was not set in icw1");
        }
        self.icw4 = Some(icw);
        self
    }

    /// Returns whether the ICW1 has the icw4_needed flag set.
    ///
    /// Panic:
    /// Will panic if no ICW1 was pushed.
    fn icw4_needed(&self) -> bool {
        self.icw1.expect("Icw1 was not provided").get_icw4_needed()
    }

    /// Returns the kind of the chip being configured: Master or Slave.
    pub fn pic_kind(&self) -> PICKind {
        self.icw3.expect("Icw3 was not provided").pic_kind()
    }

    /// Returns whether the aggregation of ICWs is complete and ready to be send.
    pub fn is_complete(&self) -> bool {
        self.icw1.is_some()
            && self.icw2.is_some()
            && self.icw3.is_some()
            && (!self.icw4_needed() || self.icw4.is_some())
    }

    /// Returns the number of cascaded lines of the master chip.
    ///
    /// Panic:
    /// Will panic if the corresponding chip, is not, in fact, a master chip.
    pub fn cascaded_lines(&self) -> usize {
        assert_eq!(self.pic_kind(), PICKind::Master);
        self.icw3.unwrap().cascaded_lines()
    }
}

/// This data structure describes a valid ICW1.
/// Sending it to a 8259 chip will start its initialization procedure.
/// One important flag is the icw4_needed flag, which tells the chip
/// to wait for a fourth ICW (ICW4).
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

/// This data structure describe a valid ICW2.
/// This ICW is used to set the interrupt vectors of the chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW2 {
    pub byte: u8,
}

impl ICW2 {
    /// Creates a new instance of an unsetup ICW2.
    pub fn new() -> Self {
        Self { byte: 0 }
    }

    /// Sets the interrupt vectors of the chip being configured.
    /// That is, for which Interrupt Entry the PIC should notify the CPU
    /// upon dispatching of one of its 8 interrupt line.
    pub fn set_interrupt_vector(mut self, vector: u8) -> Self {
        // The 3 lowest bits are not used in 8086/8088-mode.

        // vector >>= 3;
        // self.byte.set_bits(3..=7, vector);
        self.byte = vector;
        self
    }
}

/// This data structure describe a valid ICW3.
/// This ICW is used to setup the cascading mode, to tell the chip its identity;
/// that is, whether it is a slave chip or the master chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW3 {
    kind: PICKind,
    pub byte: u8,
    cascaded_lines: u8,
}

impl ICW3 {
    /// Creates an new unsetup ICW3 for a specific `kind` of chip.
    pub fn new(kind: PICKind) -> Self {
        Self {
            kind,
            byte: 0,
            cascaded_lines: 0,
        }
    }

    /// Sets an interrupt line of a master chip as a cascaded line.
    /// Only usable on a master chip.
    /// Panic:
    /// Will panic if setting an interrupt line of a slave chip is attempted.
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

    /// Sets the slave identity of a slave chip.
    /// This tells, once issued, the slave chip which interrupt line of the master chip
    /// will be used as the corresponding cascading line, i.e, for communication
    /// between the slave chip and the master chip.
    pub fn set_slave_identity(mut self, id: u8) -> Self {
        if self.kind == PICKind::Master {
            panic!("Tried to set slave identity for a Master PIC");
        } else if id > 7 {
            panic!("Invalid slave id: {}", id);
        }

        self.byte = id;
        self
    }

    /// Returns the number of cascaded lines configured for the master chip.
    pub fn cascaded_lines(&self) -> usize {
        if self.kind == PICKind::Slave {
            panic!("Only a PIC master can have cascaded lines");
        }
        self.cascaded_lines as usize
    }

    /// Returns the kind of the chip being configured: Master or Slave.
    pub fn pic_kind(&self) -> PICKind {
        self.kind
    }
}

/// This data structure describes a valid ICW4.
/// This ICW is optional and depends on initial ICW1 send to the 8259 chip.
/// It is used to setup the modes of operation of the chip, such as 8086/8088-mode
/// (the only supported for our architecture), the automatic EOI mode,
/// the buffering modes, etc...
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW4 {
    pub byte: u8,
}

impl ICW4 {
    /// Creates a new instance of an unsetup ICW4.
    pub fn new() -> Self {
        Self {
            // We only support the 8086/8088-mode
            byte: 0b1,
        }
    }

    /// Sets the Automatic EOI flag, which will set the automatic EOI mode on the chip
    /// upon issuing of the OCW.
    pub fn set_automatic_eio(mut self, value: bool) -> Self {
        self.byte.set_bit(1, value);
        self
    }

    /// Sets the buffering mode of the chip.
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

    /// Enables the special fully nested mode of the chip.
    pub fn set_special_fully_nested_mode(mut self, value: bool) -> Self {
        self.byte.set_bit(4, value);
        self
    }
}
