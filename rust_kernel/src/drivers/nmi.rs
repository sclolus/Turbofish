/// This file contains the primitives to enable and disable the NMI (Non-Maskable Interrupt)
use crate::Spinlock;
use io::{Io, Pio};
use lazy_static::lazy_static;

pub struct Nmi {
    command: Pio<u8>,
}

impl Nmi {
    const CONTROL_PORT: u16 = 0x70;
    pub const fn new() -> Self {
        Self {
            command: Pio::new(Self::CONTROL_PORT),
        }
    }

    pub fn enable() {
        let mut controller = Self::new();

        // Read the current status of the control register and mask the NMI disabled bit.
        let current_status = controller.command.read() & 0x7f;

        // Write the updated status into the control register, enabling the NMI.
        controller.command.write(current_status);
    }

    pub fn disable() {
        let mut controller = Self::new();

        // Read the current status of the control register and enable the NMI disabled bit.
        let current_status = controller.command.read() | 0x80;

        // Write the updated status into the control register,  disabling the NMI.
        controller.command.write(current_status);
    }
}
