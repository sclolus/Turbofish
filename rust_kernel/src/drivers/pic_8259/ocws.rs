//! This files contains the code related to the Operation Control Words of the PIC 8259 chip.
//! See [PIC](https://wiki.osdev.org/PIC).
//! See https://pdos.csail.mit.edu/6.828/2012/readings/hardware/8259A.pdf (Intel specification).

use super::PICRegister;
use bit_field::BitField;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct OCW1 {
    pub byte: u8,
}

impl OCW1 {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the mask of some interrupt line to `value`.
    /// When an interrupt line's mask is set, the interrupt line is disabled.
    #[allow(unused)]
    pub fn set_interrupt_mask(mut self, line: u8, value: bool) -> Self {
        assert!(line < 8, "There are only 8 lines to set in a OCW1");

        self.byte.set_bit(line as usize, value);
        self
    }

    /// Sets the masks of all the lines of the given 8259 chip (PIC).
    pub fn set_interrupt_masks(mut self, masks: u8) -> Self {
        self.byte = masks;
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct OCW2 {
    pub byte: u8,
    ir_level_set: bool,
    command_set: bool,
}

impl OCW2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_ir_level(mut self, level: u8) -> Self {
        assert!(level < 8, "There are only 8 levels to be acted upon");

        self.byte.set_bits(0..3, level);
        self.ir_level_set = true;
        self
    }

    fn set_command(mut self, value: u8) -> Self {
        assert!(value < 8, "The OCW2 commands are 3 bits values");

        self.byte.set_bits(5..=7, value);

        // could check for command already set.
        self.command_set = true;
        self
    }

    pub fn set_non_specific_eoi(self) -> Self {
        self.set_command(0b001)
    }

    #[allow(unused)]
    pub fn set_specific_eoi(self) -> Self {
        self.set_command(0b011)
    }

    #[allow(unused)]
    pub fn set_rotate_on_non_specific_eoi(self) -> Self {
        self.set_command(0b101)
    }

    #[allow(unused)]
    pub fn set_rotate_in_automatic_eoi_mode(self, value: bool) -> Self {
        if value {
            self.set_command(0b100)
        } else {
            self.set_command(0b000)
        }
    }

    #[allow(unused)]
    pub fn set_rotate_on_specific_eoi(mut self) -> Self {
        self.set_command(0b111)
    }

    #[allow(unused)]
    pub fn set_priority_command(mut self) -> Self {
        self.set_command(0b110)
    }

    #[allow(unused)]
    pub fn set_no_op(mut self) -> Self {
        self.set_command(0b010)
    }

    #[allow(unused)]
    pub fn is_complete(&self) -> bool {
        self.ir_level_set && self.command_set
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OCW3 {
    pub byte: u8,
}

impl OCW3 {
    pub fn new() -> Self {
        let mut new = Self { byte: 0 };

        // Bit 3 must be set
        new.byte.set_bit(3, true);
        new
    }

    /// Sets the register to read at next read on the data port.
    pub fn set_read_register(mut self, register: PICRegister) -> Self {
        let bits = match register {
            PICRegister::InRequest => 0b10,
            PICRegister::InService => 0b11,
            PICRegister::InterruptMasks => panic!("Can't get IMR by means of a OCW3."),
        };

        self.byte.set_bits(0..=1, bits);
        self
    }

    #[allow(unused)]
    pub fn set_poll_commmand(mut self) -> Self {
        self.byte.set_bit(2, true);
        self
    }

    /// If value is true, the OCW3 sets the special mask.
    /// else, the OCW3 clears the special mask.
    #[allow(unused)]
    pub fn set_special_mask(mut self, value: bool) -> Self {
        if value {
            self.byte.set_bits(5..=6, 0b11);
        } else {
            self.byte.set_bits(5..=6, 0b10);
        }
        self
    }
}
