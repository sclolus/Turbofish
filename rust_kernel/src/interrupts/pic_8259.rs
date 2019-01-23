//This files contains the code related to the 8259 Programmable interrupt controller

use crate::io::{_inb, _outb};

struct Pic {
    // Well. If this implementation with complete I guess there would be some data here.
}

const MASTER_COMMAND_PORT: u16 = 0x20;
const MASTER_DATA_PORT: u16 = MASTER_COMMAND_PORT + 1;

const SLAVE_COMMAND_PORT: u16 = 0xA0;
const SLAVE_DATA_PORT: u16 = SLAVE_COMMAND_PORT + 1;

// End of Interrupt
const EOI: u8 = 0x20;
const INIT: u8 = 0x11;

// Send `byte` to master's command port
fn send_to_master(byte: u8) {
    _outb(byte, MASTER_COMMAND_PORT);
}

// Send `byte` to slave's command port
fn send_to_slave(byte: u8) {
    _outb(byte, SLAVE_COMMAND_PORT);
}

// Send `byte` to master's data port
fn send_to_data_master(byte: u8) {
    _outb(byte, MASTER_DATA_PORT);
}

// Send `byte` to slave's data port
fn send_to_data_slave(byte: u8) {
    _outb(byte, SLAVE_DATA_PORT);
}

// Get the interrupt mask of the slave PIC
// WARNING: There must be no current command issued
fn get_slave_interrupt_mask() -> u8 {
    _inb(SLAVE_DATA_PORT)
}

// Get the interrupt mask of the master PIC
// WARNING: There must be no current command issued
fn get_master_interrupt_mask() -> u8 {
    _inb(MASTER_DATA_PORT)
}

// Quick explication on interrupt masks:
// the masks are one byte. Each pic has one.
// The bits of the masks correspond to the interrupts lines.
// Each pic having 8 interrupts lines, when one bit is set in the IMR,
// the corresponding interrupt line is disabled. (ignored by the PIC).
// WARNING: The IRQ line 2 of the master is the line used to receive the slave's interrupts.
// Setting it will disable all the slave's interrupts.

// Set the slave's IMR (interrupt mask register) to mask
fn set_slave_interrupt_mask(mask: u8) {
    send_to_data_slave(mask);
}

// Set the master's IMR (interrupt mask register) to mask
fn set_master_interrupt_mask(mask: u8) {
    send_to_data_master(mask);
}


// This function will set the bit `irq`.
// Disabling the corresponding interrupt line.
// if irq < 8, then the master mask is modified.
// if irq >= 8 then the slave mask is modified.
pub fn irq_set_mask(mut irq: u8) {
    assert!(irq < 16);
    if irq < 8 {
        let mask = get_master_interrupt_mask() | (0x1 << irq);

        set_master_interrupt_mask(mask);
    } else {
        irq -= 8;
        let mask = get_slave_interrupt_mask() | (0x1 << irq);

        set_slave_interrupt_mask(mask);
    }
}

// This function will clear the bit `irq`.
// Enabling the corresopnding interrupt line.
// if irq < 8, then the master mask is modified.
// if irq >= 8 then the slave mask is modified.
pub fn irq_clear_mask(mut irq: u8) {
    assert!(irq < 16);
    if irq < 8 {
        let mask = get_master_interrupt_mask() & !(0x1 << irq);

        set_master_interrupt_mask(mask);
    } else {
        irq -= 8;
        let mask = get_slave_interrupt_mask() & !(0x1 << irq);

        set_slave_interrupt_mask(mask);
    }
}

// Disable both Slave and Master PICs
// This is done by sending 0xff to their respective data ports
pub fn disable_pics() {
    set_slave_interrupt_mask(0xff);
    set_master_interrupt_mask(0xff);
}

// Send end of interrupt from specific IRQ to the PIC.
// If the interrupt line is handled by the master chip,
// only to him the eoi is send.
// If the interrupt line is handled by the slave chip,
// both the slave and the master must be notified.
pub fn send_eoi(irq: u8) {
    assert!(irq < 16);
    if irq >= 8 {
        send_to_slave(EOI);
    }
    send_to_master(EOI);
}

// Initialize the PICs with `offset_1` as the vector offset for master
// and `offset_2` as the vector offset for slave.
// Which means that the vectors for master are now: offset_1..=offset_1+7
// and for slave: offset_2..=offset_2+7.
pub fn initialize(offset_1: u8, offset_2: u8) {
    let slave_mask = get_slave_interrupt_mask();
    let master_mask = get_master_interrupt_mask();

    send_to_master(INIT);
    send_to_slave(INIT);
    send_to_data_master(offset_1);
    send_to_data_slave(offset_2);
    send_to_data_master(4); // This tells the master that there is a slave at its IRQ2
    send_to_data_slave(2); // This tells the slave its cascade identity

    // thoses 2 calls set the 8086/88 (MCS-80/85) mode for master and slave.
    send_to_data_master(1);
    send_to_data_slave(1);

    set_slave_interrupt_mask(slave_mask);
    set_master_interrupt_mask(master_mask);
}
