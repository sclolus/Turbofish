//This files contains the code related to the 8259 Programmable interrupt controller

use crate::io::{Io, Pio};

#[allow(non_upper_case_globals)]
pub static mut master: Pic = Pic::new(Pic::MASTER_COMMAND_PORT);
#[allow(non_upper_case_globals)]
pub static mut slave: Pic = Pic::new(Pic::SLAVE_COMMAND_PORT);

pub struct Pic {
    command: Pio<u8>,
    data: Pio<u8>,
}

impl Pic {
    const MASTER_COMMAND_PORT: u16 = 0x20;
    const SLAVE_COMMAND_PORT: u16 = 0xA0;
    
    // End of Interrupt
    const EOI: u8 = 0x20;
    const INIT: u8 = 0x11;
    const PIC_READ_IRR: u8 = 0x0a;
    const PIC_READ_ISR: u8 = 0x0b;


    pub const fn new(port: u16) -> Pic {
        Pic {
            command: Pio::new(port),
            data: Pio::new(port + 1),
        }
    }

    // Get the interrupt mask of the slave PIC
    // WARNING: There must be no current command issued
    pub fn get_interrupt_mask(&self) -> u8 {
        self.data.read()
    }

    // Quick explication on interrupt masks:
    // the masks are one byte. Each pic has one.
    // The bits of the masks correspond to the interrupts lines.
    // Each pic having 8 interrupts lines, when one bit is set in the IMR,
    // the corresponding interrupt line is disabled. (ignored by the PIC).
    // WARNING: The IRQ line 2 of the master is the line used to receive the slave's interrupts.
    // Setting it will disable all the slave's interrupts.

    pub fn set_interrupt_mask(&mut self, mask: u8) {
        self.data.write(mask)
    }
}

// This function will set the bit `irq`.
// Disabling the corresponding interrupt line.
// if irq < 8, then the master mask is modified.
// if irq >= 8 then the slave mask is modified.
pub unsafe fn irq_set_mask(mut irq: u8) {
    assert!(irq < 16);
    if irq < 8 {
        let mask = master.get_interrupt_mask() | (0x1 << irq);

        master.set_interrupt_mask(mask);
    } else {
        irq -= 8;
        let mask = slave.get_interrupt_mask() | (0x1 << irq);

        slave.set_interrupt_mask(mask);
    }
}

// This function will clear the bit `irq`.
// Enabling the corresopnding interrupt line.
// if irq < 8, then the master mask is modified.
// if irq >= 8 then the slave mask is modified.
pub unsafe fn irq_clear_mask(mut irq: u8) {
    assert!(irq < 16);
    if irq < 8 {
        let mask = master.get_interrupt_mask() & !(0x1 << irq);

        master.set_interrupt_mask(mask);
    } else {
        irq -= 8;
        let mask = slave.get_interrupt_mask() & !(0x1 << irq);

        slave.set_interrupt_mask(mask);
    }
}

// Disable both Slave and Master PICs
// This is done by sending 0xff to their respective data ports
pub unsafe fn disable_pics() {
    master.set_interrupt_mask(0xff);
    slave.set_interrupt_mask(0xff);
}

pub unsafe fn enable_all_interrupts() {
    master.set_interrupt_mask(0x0);
    slave.set_interrupt_mask(0x0);
}

// Send end of interrupt from specific IRQ to the PIC.
// If the interrupt line is handled by the master chip,
// only to him the eoi is send.
// If the interrupt line is handled by the slave chip,
// both the slave and the master must be notified.
pub fn send_eoi(irq: u8) {
    unsafe {
        assert!(irq < 16);
        
        if irq >= 8 {
            slave.command.write(Pic::EOI);
        }
        master.command.write(Pic::EOI);
    }
}

// Initialize the PICs with `offset_1` as the vector offset for master
// and `offset_2` as the vector offset for slave.
// Which means that the vectors for master are now: offset_1..=offset_1+7
// and for slave: offset_2..=offset_2+7.
pub unsafe fn initialize(offset_1: u8, offset_2: u8) {
    let slave_mask = slave.get_interrupt_mask();
    let master_mask = master.get_interrupt_mask();

    master.command.write(Pic::INIT);

    slave.command.write(Pic::INIT);

    // Assign the vectors offsets
    master.data.write(offset_1);
    slave.data.write(offset_2);
    
    master.data.write(4); // This tells the master that there is a slave at its IRQ2
    slave.data.write(2); // This tells the slave its cascade identity

    // thoses 2 calls set the 8086/88 (MCS-80/85) mode for master and slave.
    master.data.write(1);
    slave.data.write(1);

    // Reset all interrupt masks
    slave.set_interrupt_mask(slave_mask);
    master.set_interrupt_mask(master_mask);
}

unsafe fn pic_get_irq_reg(ocw3: u8) -> u16 {
    master.command.write(ocw3);
    slave.command.write(ocw3);

    (slave.command.read() as u16) << 8 | master.command.read() as u16
}

// Returns the combined value the PICs irq request register
pub fn get_irr() -> u16 {
    unsafe {
        pic_get_irq_reg(Pic::PIC_READ_IRR)
    }
}

// Returns the combined value the PICs irq request register
pub fn get_isr() -> u16 {
    unsafe {
        pic_get_irq_reg(Pic::PIC_READ_ISR)
    }
}
