//! This crate provide a small brief about IRQ
#![cfg_attr(not(test), no_std)]

#[derive(Debug, Copy, Clone)]
pub enum Irq {
    /// The System timer, (PIT: Programmable Interval Timer) IRQ.
    SystemTimer = 0,

    /// The Keyboard Controller IRQ.
    KeyboardController = 1,

    /// IRQ 2 – cascaded signals from IRQs 8–15 (any devices configured to use IRQ 2 will actually be using IRQ 9)
    SlaveCascadeIRQ = 2,

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
