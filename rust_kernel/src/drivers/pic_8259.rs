//! This files contains the code related to the 8259 Programmable interrupt controller.
//! See [PIC](https://wiki.osdev.org/PIC).
//! See https://pdos.csail.mit.edu/6.828/2012/readings/hardware/8259A.pdf (Intel specification).

mod pic_8259_isr;
use crate::Spinlock;
use bit_field::BitField;
use io::{Io, Pio};
use itertools::unfold;
use lazy_static::lazy_static;
use pic_8259_isr::*;

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

    configuration: Option<ICWs>,
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
        Pic {
            command: Pio::new(port),
            data: Pio::new(port + 1),
            configuration: None,
        }
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

    pub fn configure(&mut self, config: ICWs) {
        assert!(config.is_complete());
        println!("ICW1: {:x}", config.icw1.unwrap().byte);
        println!("ICW2: {:x}", config.icw2.unwrap().byte);
        println!("ICW3: {:x}", config.icw3.unwrap().byte);
        println!("ICW4: {:x}", config.icw4.unwrap().byte);

        self.command.write(config.icw1.unwrap().byte);
        self.data.write(config.icw2.unwrap().byte);
        self.data.write(config.icw3.unwrap().byte);
        self.data.write(config.icw4.unwrap().byte);
        self.configuration = Some(config);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW1 {
    byte: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TriggeringMode {
    Level,
    Edge,
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

    pub fn get_icw4_needed(mut self) -> bool {
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
pub enum PICKind {
    Master,
    Slave,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW2 {
    byte: u8,
}

impl ICW2 {
    pub fn new() -> Self {
        Self { byte: 0 }
    }

    pub fn set_interrupt_vector(mut self, mut vector: u8) -> Self {
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
    byte: u8,
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
            panic!("Only a PIC master can have cascaded lines"); // check this
        }
        self.cascaded_lines as usize
    }

    pub fn pic_kind(&self) -> PICKind {
        self.kind
    }

    // set slave id
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ICW4 {
    byte: u8,
}

pub enum BufferingMode {
    NotBuffered,
    SlaveBuffered,
    MasterBuffered,
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
    icw1: Option<ICW1>,
    icw2: Option<ICW2>,
    icw3: Option<ICW3>,
    icw4: Option<ICW4>,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PicConfiguration {
    configurations: [Option<ICWs>; 8],
    nbr_pics: usize,
    master_index: Option<usize>,
}

impl PicConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pic_configuration(&mut self, config: ICWs) -> &mut Self {
        if self.nbr_pics == 8 {
            panic!("A master pic can only be cascaded with a maximum of 8 slaves");
        }

        if !config.is_complete() {
            panic!("Pic Configuration must be complete");
        }

        if config.pic_kind() == PICKind::Master {
            assert!(
                self.master_index.is_none(),
                "There can only be one master PIC"
            );
            self.master_index = Some(self.nbr_pics);
        }

        self.configurations[self.nbr_pics] = Some(config);
        self.nbr_pics += 1;
        self
    }

    pub fn has_master(&self) -> bool {
        self.master_index.is_some()
    }

    pub fn get_master(&self) -> &ICWs {
        assert!(self.has_master(), "No PIC master were found.");
        self.configurations[self.master_index.unwrap()]
            .as_ref()
            .unwrap()
    }

    pub fn nbr_pics(&self) -> usize {
        self.nbr_pics
    }

    pub fn is_complete(&self) -> bool {
        return self.nbr_pics != 0
            && self.has_master()
            && self.get_master().cascaded_lines() == self.nbr_pics.checked_sub(1).unwrap_or(0);
    }

    pub fn slaves<'a>(&'a self) -> impl Iterator<Item = ICWs> + 'a {
        let mut slaves = self
            .configurations
            .iter()
            .filter_map(|&c| c)
            .filter(|&c| c.pic_kind() == PICKind::Slave);
        unfold((), move |_| slaves.next())
    }
}

pub struct Pic8259 {
    master: Pic,
    slave: Pic,
    bios_imr: Option<u16>,
}

lazy_static! {
    pub static ref PIC_8259: Spinlock<Pic8259> = Spinlock::new(Pic8259::new());
}

#[derive(Debug, Copy, Clone)]
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

    /// Those are the current default handlers for the IRQs from the PICs 8259 (master)
    /// They are mapped from 0x20 to 0x27
    const DEFAULT_IRQS_MASTER: [unsafe extern "C" fn(); 8] = [
        _isr_timer,
        _isr_keyboard,
        _isr_cascade,
        _isr_com2,
        _isr_com1,
        _isr_lpt2,
        _isr_floppy_disk,
        _isr_lpt1,
    ];

    /// Those are the current default handlers for the IRQs from the PICs 8259 (slave)
    /// They are mapped from 0x28 to 0x30
    const DEFAULT_IRQS_SLAVE: [unsafe extern "C" fn(); 8] = [
        _isr_cmos,
        _isr_acpi,
        reserved_interruption,
        reserved_interruption,
        _isr_ps2_mouse,
        _isr_fpu_coproc,
        _isr_primary_hard_disk,
        _isr_secondary_hard_disk,
    ];

    pub const fn new() -> Self {
        Self {
            master: Pic::new(Self::MASTER_COMMAND_PORT),
            slave: Pic::new(Self::SLAVE_COMMAND_PORT),
            bios_imr: None,
        }
    }

    /// Must be called when PIC is initialized
    /// The bios default IMR are stored when this function is called
    pub unsafe fn init(&mut self) {
        let mut interrupt_table = InterruptTable::current_interrupt_table().unwrap();

        self.bios_imr = Some(self.get_masks());
        self.set_idt_vectors(KERNEL_PIC_MASTER_IDT_VECTOR, KERNEL_PIC_SLAVE_IDT_VECTOR);

        use crate::interrupts::idt::GateType::InterruptGate32;
        use crate::interrupts::idt::*;
        use core::ffi::c_void;

        let mut gate_entry = *IdtGateEntry::new()
            .set_storage_segment(false)
            .set_privilege_level(0)
            .set_selector(1 << 3)
            .set_gate_type(InterruptGate32);

        gate_entry.set_gate_type(InterruptGate32);

        let offset = KERNEL_PIC_MASTER_IDT_VECTOR as usize;
        for (index, &interrupt_handler) in Self::DEFAULT_IRQS_MASTER.iter().enumerate() {
            gate_entry.set_handler(interrupt_handler as *const c_void as u32);

            interrupt_table[index + offset] = gate_entry;
        }

        let offset = KERNEL_PIC_SLAVE_IDT_VECTOR as usize;
        for (index, &interrupt_handler) in Self::DEFAULT_IRQS_SLAVE.iter().enumerate() {
            gate_entry.set_handler(interrupt_handler as *const c_void as u32);

            interrupt_table[index + offset] = gate_entry;
        }
    }

    /// Public ascessor to check init of PIC
    pub fn is_initialized(&self) -> bool {
        self.bios_imr != None
    }

    pub fn default_pit_configuration() -> PicConfiguration {
        use PICKind::*;

        let master_icw1 = ICW1::new().set_icw4_needed(true);
        let master_icw2 = ICW2::new().set_interrupt_vector(KERNEL_PIC_MASTER_IDT_VECTOR);
        let master_icw3 = ICW3::new(Master).set_cascaded_line(2, true);
        let master_icw4 = ICW4::new();

        let master_icws = ICWs::new()
            .push_icw1(master_icw1)
            .push_icw2(master_icw2)
            .push_icw3(master_icw3)
            .push_icw4(master_icw4);

        let slave_icw1 = ICW1::new().set_icw4_needed(true);
        let slave_icw2 = ICW2::new().set_interrupt_vector(KERNEL_PIC_SLAVE_IDT_VECTOR);
        let slave_icw3 = ICW3::new(Slave).set_slave_identity(2);
        let slave_icw4 = ICW4::new();

        let slave_icws = ICWs::new()
            .push_icw1(slave_icw1)
            .push_icw2(slave_icw2)
            .push_icw3(slave_icw3)
            .push_icw4(slave_icw4);

        let mut configuration = PicConfiguration::new();

        configuration
            .add_pic_configuration(master_icws)
            .add_pic_configuration(slave_icws);

        configuration
    }

    pub fn initialize(&mut self, pic_configuration: PicConfiguration) {
        assert!(pic_configuration.is_complete());
        assert!(
            pic_configuration.nbr_pics() == 2,
            "Currently only one master and one slave are supported"
        );

        let master_conf = pic_configuration.get_master();
        let mut slaves_confs = pic_configuration.slaves();

        self.master.configure(*master_conf);
        for slave_conf in slaves_confs {
            self.slave.configure(slave_conf);
        }
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
        self.master.data.write(0b1);
        self.slave.data.write(0b1);
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
        unsafe {
            (self.master.get_interrupt_mask() as u16)
                | ((self.slave.get_interrupt_mask() as u16) << 8)
        }
    }

    /// Send end of interrupt from specific IRQ to the PIC.
    /// If the interrupt line is handled by the self.master chip, only to him the eoi is send.
    /// If the interrupt line is handled by the self.slave chip, both the self.slave and the self.master must be notified.
    pub fn send_eoi(&mut self, irq: Irq) {
        let nirq = irq as u16;
        assert!(nirq < 16);
        if nirq >= 8 {
            self.slave.command.write(Pic::EOI);
        }
        self.master.command.write(Pic::EOI);
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
