//! This file contains the primitives to enable and disable the NMI (Non-Maskable Interrupt)
use io::{Io, Pio};

/// Boilerplate structure to enable and disable the Non-Maskable Interrupt.
pub struct Nmi {
    command: Pio<u8>,
}

impl Nmi {
    const CONTROL_PORT: u16 = 0x70;
    const NMI_DISABLE_BIT: u8 = 0x80;
    pub const fn new() -> Self {
        Self { command: Pio::new(Self::CONTROL_PORT) }
    }

    /// Enables the NMI.
    pub fn enable() {
        let mut controller = Self::new();

        // Read the current status of the control register and mask the NMI disabled bit.
        let current_status = controller.command.read() & !Self::NMI_DISABLE_BIT;

        // Write the updated status into the control register, enabling the NMI.
        controller.command.write(current_status);
    }

    /// Disables the NMI.
    pub fn disable() {
        let mut controller = Self::new();

        // Read the current status of the control register and enable the NMI disabled bit.
        let current_status = controller.command.read() | Self::NMI_DISABLE_BIT;

        // Write the updated status into the control register,  disabling the NMI.
        controller.command.write(current_status);
    }
}
