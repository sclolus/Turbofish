//! This files contains the code related to the 8259 Programmable interrupt controller.
//! See [PIC](https://wiki.osdev.org/PIC)
use crate::io::{Io, Pio};
use bit_field::BitField;

const BIOS_PIC_MASTER_IDT_VECTOR: u8 = 0x08 as u8;
const BIOS_PIC_SLAVE_IDT_VECTOR: u8 = 0x70 as u8;

pub const KERNEL_PIC_MASTER_IDT_VECTOR: u8 = 0x20 as u8;
pub const KERNEL_PIC_SLAVE_IDT_VECTOR: u8 = 0x28 as u8;

/// Represents a Programmable Interrupt Controller 8259
pub struct Pic {
    /// The PIC's command port.
    command: Pio<u8>,

    /// The PIC's data port.
    data: Pio<u8>,
}

impl Pic {
    /// The End of Interrupt command, used to reply to the PICs at the end of an interrupt handler
    const EOI: u8 = 0x20;

    /// The Initialization command, used to start the initialization of the PICs
    const INIT: u8 = 0x11;

    /// The Read Interrupt Request Register command, used to obtain the Interrupt Request Register from the PICs
    const PIC_READ_IRR: u8 = 0x0a;

    /// The In-Service Register command, used to obtain the In-Service Register from the PICs.
    const PIC_READ_ISR: u8 = 0x0b;

    /// Creates a new PIC instance with port `port`
    pub const fn new(port: u16) -> Self {
        Pic { command: Pio::new(port), data: Pio::new(port + 1) }
    }

    /// Get the interrupt mask of the slave PIC
    /// # Warning:
    /// There must be no current command issued
    pub unsafe fn get_interrupt_mask(&self) -> u8 {
        self.data.read()
    }

    /// Quick explication on interrupt masks:
    /// the masks are one byte. Each pic has one.
    /// The bits of the masks correspond to the interrupts lines.
    /// Each pic having 8 interrupts lines, when one bit is set in the IMR,
    /// the corresponding interrupt line is disabled. (ignored by the PIC).
    /// # Warning:
    /// The IRQ line 2 of the master is the line used to receive the slave's interrupts.
    /// Setting it will disable all the slave's interrupts.
    pub unsafe fn set_interrupt_mask(&mut self, mask: u8) {
        self.data.write(mask)
    }
}

pub struct Pic8259 {
    master: Pic,
    slave: Pic,
    bios_imr: Option<u16>,
}

pub static mut PIC_8259: Pic8259 = Pic8259::new();

pub enum Irq {
    /// The System timer, (PIT: Programmable Interval Timer) IRQ.
    SystemTimer = 0,

    /// The Keyboard Controller IRQ.
    KeyboardController = 1,

    //IRQ 2 – cascaded signals from IRQs 8–15 (any devices configured to use IRQ 2 will actually be using IRQ 9)
    /// The Serial Port 2 IRQ (shared with the Serial Port 4, if it is present).
    SerialPortController2 = 3,

    /// The Serial Port 1 IRQ (shared with the Serial Port 3, if it is present).
    SerialPortController1 = 4,

    /// The IRQ for Parallel Ports 2 and 3, or the Sound card.
    ParallelPort2And3 = 5, //  or  sound card

    /// The IRQ for the FloppyDisk Controller.
    FloppyDiskController = 6,

    /// The IRQ for the Parallel Port 1.
    /// Note: It is used for printers or for any parallel port if a printer is not present. It can also be potentially be shared with a secondary sound card with careful management of the port.
    ParallelPort1 = 7,

    /// The Real Time Clock (RTC) IRQ.
    RealTimeClock = 8, // (RTC)

    /// The IRQ for the Advanced Configuration and Power Interface (on Intel chips, mostly).
    ACPI = 9,

    /// The IRQ is left open for the use of peripherals (open interrupt/available, SCSI or NIC).
    Irq10 = 10,

    /// The IRQ is left open for the use of peripherals (open interrupt/available, SCSI or NIC)
    Irq11 = 11,

    /// The IRQ for the mouse from the PS/2 Controller.
    MouseOnPS2Controller = 12,

    /// The IRQ for CPU co-processor or integrated floating point unit or inter-processor interrupt (use depends on OS).
    Irq13 = 13,

    /// The IRQ for the Primary ATA Channel.
    /// (ATA interface usually serves hard disk drives and CD drives)
    PrimaryATAChannel = 14,

    /// The IRQ for the Secondary ATA Channel.
    /// (ATA interface usually serves hard disk drives and CD drives)
    SecondaryATAChannel = 15,
}

impl Pic8259 {
    /// The default port number for the master PIC
    const MASTER_COMMAND_PORT: u16 = 0x20;

    /// The default port number for the slave PIC
    const SLAVE_COMMAND_PORT: u16 = 0xA0;

    pub const fn new() -> Self {
        Self { master: Pic::new(Self::MASTER_COMMAND_PORT), slave: Pic::new(Self::SLAVE_COMMAND_PORT), bios_imr: None }
    }

    /// Must be called when PIC is initialized
    /// The bios default IMR are stored when this function is called
    pub unsafe fn init(&mut self) {
        self.bios_imr = Some(self.get_masks());
        self.set_idt_vectors(KERNEL_PIC_MASTER_IDT_VECTOR, KERNEL_PIC_SLAVE_IDT_VECTOR);
    }

    /// Initialize the PICs with `offset_1` as the vector offset for self.master
    /// and `offset_2` as the vector offset for self.slave.
    /// Which means that the vectors for self.master are now: offset_1..=offset_1+7
    /// and for self.slave: offset_2..=offset_2+7.
    pub unsafe fn set_idt_vectors(&mut self, offset_1: u8, offset_2: u8) {
        self.master.command.write(Pic::INIT);
        self.slave.command.write(Pic::INIT);

        // Assign the vectors offsets
        self.master.data.write(offset_1);
        self.slave.data.write(offset_2);

        self.master.data.write(0b100); // This tells the self.master that there is a self.slave at its IRQ2
        self.slave.data.write(0b10); // This tells the self.slave its cascade identity

        // thoses 2 calls set the 8086/88 (MCS-80/85) mode for self.master and self.slave.
        self.master.data.write(1);
        self.slave.data.write(1);
    }

    /// This function will set the bit `irq`.
    /// Disabling the corresponding interrupt line.
    /// if irq < 8, then the self.master mask is modified.
    /// if irq >= 8 then the self.slave is modified.
    pub unsafe fn disable_irq(&mut self, irq: Irq) {
        let mut nirq = irq as usize;
        assert!(nirq < 16);
        if nirq < 8 {
            let mask = *self.master.get_interrupt_mask().set_bit(nirq, true);

            self.master.set_interrupt_mask(mask);
        } else {
            nirq -= 8;
            let mask = *self.slave.get_interrupt_mask().set_bit(nirq, true);

            self.slave.set_interrupt_mask(mask);
        }
    }

    /// This function will clear the bit `irq`.
    /// Enabling the corresponding interrupt line.
    /// if irq < 8, then the self.master mask is modified.
    /// if irq >= 8 then the self.slave and master mask is modified.
    pub unsafe fn enable_irq(&mut self, irq: Irq) {
        let mut nirq = irq as usize;
        assert!(nirq < 16);
        if nirq < 8 {
            let mask = *self.master.get_interrupt_mask().set_bit(nirq, false);

            self.master.set_interrupt_mask(mask);
        } else {
            nirq -= 8;
            let mask = *self.slave.get_interrupt_mask().set_bit(nirq, false);

            self.slave.set_interrupt_mask(mask);

            // Also clear irq 2 to enable slave sending to master
            let mask = *self.master.get_interrupt_mask().set_bit(2, false);

            self.master.set_interrupt_mask(mask);
        }
    }

    /// Disable both Slave and Master PICs
    /// This is done by sending 0xff to their respective data ports
    pub unsafe fn disable_all_irqs(&mut self) {
        self.master.set_interrupt_mask(0xff);
        self.slave.set_interrupt_mask(0xff);
    }

    /// Enable all interrupts of the PICs by clearing their Interrupt Mask
    pub unsafe fn enable_all_irqs(&mut self) {
        self.master.set_interrupt_mask(0x0);
        self.slave.set_interrupt_mask(0x0);
    }

    /// Restores the IMRs of the self.master and self.slave PICs to the combined `mask` parameter
    /// The bits 0 to 7 (inclusive) are the self.master's IMR.
    /// The bits 8 to 15 (inclusive) are the self.slave's IMR.
    pub unsafe fn set_masks(&mut self, mask: u16) {
        self.master.set_interrupt_mask(mask.get_bits(0..8) as u8);
        self.slave.set_interrupt_mask(mask.get_bits(8..16) as u8);
    }

    /// Gets the combined IMRs of the self.master and self.slave PICs
    /// The bits 0 to 7 (inclusive) are the self.master's IMR.
    /// The bits 8 to 15 (inclusive) are the self.slave's IMR.
    pub fn get_masks(&mut self) -> u16 {
        unsafe { (self.master.get_interrupt_mask() as u16) | ((self.slave.get_interrupt_mask() as u16) << 8) }
    }

    /// Send end of interrupt from specific IRQ to the PIC.
    /// If the interrupt line is handled by the self.master chip, only to him the eoi is send.
    /// If the interrupt line is handled by the self.slave chip, both the self.slave and the self.master must be notified.
    pub fn send_eoi(&mut self, irq: Irq) {
        let nirq = irq as u16;
        assert!(nirq < 16);
        unsafe {
            if nirq >= 8 {
                self.slave.command.write(Pic::EOI);
            }
            self.master.command.write(Pic::EOI);
        }
    }

    /// Reset the PICs to the defaults IMR and irq vector offsets
    /// Returning the combined IMRs of the PICs before the reset
    /// WARNING: This fonction should not be called if the PICs were never initialized as it would panic.
    pub unsafe fn reset_to_default(&mut self) -> u16 {
        without_interrupts!({
            let imrs = self.get_masks();

            self.set_idt_vectors(BIOS_PIC_MASTER_IDT_VECTOR, BIOS_PIC_SLAVE_IDT_VECTOR);
            self.set_masks(self.bios_imr.expect("The PIC default imr was never saved"));

            imrs
        })
    }

    unsafe fn pic_get_irq_reg(&mut self, ocw3: u8) -> u16 {
        self.master.command.write(ocw3);
        self.slave.command.write(ocw3);

        (self.slave.command.read() as u16) << 8 | self.master.command.read() as u16
    }

    /// Returns the combined value the PICs irq request register
    pub fn get_irr(&mut self) -> u16 {
        unsafe { self.pic_get_irq_reg(Pic::PIC_READ_IRR) }
    }

    /// Returns the combined value the PICs in-service register
    pub fn get_isr(&mut self) -> u16 {
        unsafe { self.pic_get_irq_reg(Pic::PIC_READ_ISR) }
    }
}
