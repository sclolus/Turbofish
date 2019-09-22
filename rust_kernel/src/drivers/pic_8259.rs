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

pub use irq::Irq;

mod icws;
use icws::{ICWs, ICW1, ICW2, ICW3, ICW4};

mod ocws;
use ocws::{OCW1, OCW2, OCW3};

const BIOS_PIC_MASTER_IDT_VECTOR: u8 = 0x08 as u8;
const BIOS_PIC_SLAVE_IDT_VECTOR: u8 = 0x70 as u8;

pub const KERNEL_PIC_MASTER_IDT_VECTOR: u8 = 0x20 as u8;
pub const KERNEL_PIC_SLAVE_IDT_VECTOR: u8 = 0x28 as u8;

lazy_static! {
    pub static ref PIC_8259: Spinlock<Pic8259> = Spinlock::new(Pic8259::new());
}

pub struct Pic8259 {
    master: Pic,
    slave: Pic,
    bios_imr: Option<u16>,
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
        let mut interrupt_table = InterruptTable::current_interrupt_table();

        self.bios_imr = Some(self.get_masks());

        let default_conf = Self::default_pic_configuration();
        self.initialize(default_conf);
        self.disable_all_irqs();

        use core::ffi::c_void;
        use interrupts::idt::GateType::InterruptGate32;
        use interrupts::idt::*;

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

    /// Returns a default PicConfiguration for two 8259 chips, a master and a slave,
    /// with their respective interrupt vectors set to `master_vector` and `slave_vector`.
    pub fn basic_pic_configuration(master_vector: u8, slave_vector: u8) -> PicConfiguration {
        use PICKind::*;

        let master_icw1 = ICW1::new().set_icw4_needed(true);
        let master_icw2 = ICW2::new().set_interrupt_vector(master_vector);
        let master_icw3 = ICW3::new(Master).set_cascaded_line(2, true);
        let master_icw4 = ICW4::new();

        let master_icws = ICWs::new()
            .push_icw1(master_icw1)
            .push_icw2(master_icw2)
            .push_icw3(master_icw3)
            .push_icw4(master_icw4);

        let slave_icw1 = ICW1::new().set_icw4_needed(true);
        let slave_icw2 = ICW2::new().set_interrupt_vector(slave_vector);
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

    /// Returns a default bios PicConfiguration (Empirical).
    pub fn bios_pic_configuration() -> PicConfiguration {
        Self::basic_pic_configuration(BIOS_PIC_MASTER_IDT_VECTOR, BIOS_PIC_SLAVE_IDT_VECTOR)
    }

    /// Returns the default PicConfiguration used by our kernel.
    pub fn default_pic_configuration() -> PicConfiguration {
        Self::basic_pic_configuration(KERNEL_PIC_MASTER_IDT_VECTOR, KERNEL_PIC_SLAVE_IDT_VECTOR)
    }

    /// Initialize the PICs with the given `pic_configuration`.
    /// This solely executes the initialization procedures of the described chips.
    /// This does not set the corresponding Interruptions into the InterruptTable.
    /// Refer to the init method for such features.
    pub fn initialize(&mut self, pic_configuration: PicConfiguration) {
        assert!(pic_configuration.is_complete());
        assert!(
            pic_configuration.nbr_pics() == 2,
            "Currently only one master and one slave are supported"
        );

        let master_conf = pic_configuration.get_master();
        let slaves_confs = pic_configuration.slaves();

        self.master.configure(*master_conf);
        for slave_conf in slaves_confs {
            self.slave.configure(slave_conf);
        }
    }

    /// This function will set the bit `irq`.
    /// Disabling the corresponding interrupt line.
    /// if irq < 8, then the self.master mask is modified.
    /// if irq >= 8 then the self.slave is modified.
    pub unsafe fn disable_irq(&mut self, irq: Irq) {
        log::info!("Pic8259: Disable irq {:?}", irq);

        let mut nirq = irq as usize;
        assert!(nirq < 16);
        if nirq < 8 {
            let mask = *self.master.get_interrupt_masks().set_bit(nirq, true);

            self.master.set_interrupt_masks(mask);
        } else {
            nirq -= 8;
            let mask = *self.slave.get_interrupt_masks().set_bit(nirq, true);

            self.slave.set_interrupt_masks(mask);
        }
    }

    /// This function will clear the bit `irq`.
    /// Enabling the corresponding interrupt line.
    /// if irq < 8, then the self.master mask is modified.
    /// if irq >= 8 then the self.slave and master mask is modified.
    /// When used without function option. the default symbol in asm file is called
    pub unsafe fn enable_irq(&mut self, irq: Irq, func_opt: Option<unsafe extern "C" fn()>) {
        log::info!("Pic8259: Enable irq {:?}", irq);
        if let Some(func) = func_opt {
            log::info!("Pic8259: Assigning function at {:?}", func);
            _pic_handlers_array[irq as usize] = func as u32;
        }

        let mut nirq = irq as usize;
        assert!(nirq < 16);
        if nirq < 8 {
            let mask = *self.master.get_interrupt_masks().set_bit(nirq, false);

            self.master.set_interrupt_masks(mask);
        } else {
            nirq -= 8;
            let mask = *self.slave.get_interrupt_masks().set_bit(nirq, false);

            self.slave.set_interrupt_masks(mask);

            // Also clear irq 2 to enable slave sending to master
            let mask = *self.master.get_interrupt_masks().set_bit(2, false);

            self.master.set_interrupt_masks(mask);
        }
    }

    /// Disable both Slave and Master PICs
    /// This is done by sending 0xff to their respective data ports
    pub unsafe fn disable_all_irqs(&mut self) {
        self.master.set_interrupt_masks(0xff);
        self.slave.set_interrupt_masks(0xff);
    }

    /// Enable all interrupts of the PICs by clearing their Interrupt Mask
    pub unsafe fn enable_all_irqs(&mut self) {
        self.master.set_interrupt_masks(0x0);
        self.slave.set_interrupt_masks(0x0);
    }

    /// Restores the IMRs of the self.master and self.slave PICs to the combined `mask` parameter
    /// The bits 0 to 7 (inclusive) are the self.master's IMR.
    /// The bits 8 to 15 (inclusive) are the self.slave's IMR.
    pub unsafe fn set_masks(&mut self, mask: u16) {
        self.master.set_interrupt_masks(mask.get_bits(0..8) as u8);
        self.slave.set_interrupt_masks(mask.get_bits(8..16) as u8);
    }

    /// Send end of interrupt from specific IRQ to the PIC.
    /// If the interrupt line is handled by the self.master chip, only to him the eoi is send.
    /// If the interrupt line is handled by the self.slave chip, both the self.slave and the self.master must be notified.
    pub fn send_eoi(&mut self, irq: Irq) {
        let nirq = irq as u16;
        assert!(nirq < 16);
        if nirq >= 8 {
            // Should we send specific EOIs ?
            unsafe {
                self.slave.send_non_specific_eoi();
            }
        }
        unsafe {
            self.master.send_non_specific_eoi();
        }
    }

    /// Reset the PICs to the defaults IMR and irq vector offsets
    /// Returning the combined IMRs of the PICs before the reset
    /// WARNING: This fonction should not be called if the PICs were never initialized as it would panic.
    pub unsafe fn reset_to_default(&mut self) -> u16 {
        without_interrupts!({
            let imrs = self.get_masks();

            let bios_pic_configuration = Self::bios_pic_configuration();

            self.initialize(bios_pic_configuration);
            self.set_masks(self.bios_imr.expect("The PIC default imr was never saved"));

            imrs
        })
    }

    fn get_pics_register(&mut self, register: PICRegister) -> u16 {
        (self.slave.get_register(register) as u16) << 8 | self.master.get_register(register) as u16
    }

    /// Returns the combined value the PICs irq request register
    pub fn get_irr(&mut self) -> u16 {
        self.get_pics_register(PICRegister::InRequest)
    }

    /// Returns the combined value the PICs in-service register
    pub fn get_isr(&mut self) -> u16 {
        self.get_pics_register(PICRegister::InService)
    }

    /// Gets the combined IMRs of the self.master and self.slave PICs
    /// The bits 0 to 7 (inclusive) are the self.master's IMR.
    /// The bits 8 to 15 (inclusive) are the self.slave's IMR.
    pub fn get_masks(&mut self) -> u16 {
        self.get_pics_register(PICRegister::InterruptMasks)
    }
}

/// Represents a Programmable Interrupt Controller 8259
pub struct Pic {
    /// The PIC's command port.
    command: Pio<u8>,

    /// The PIC's data port.
    data: Pio<u8>,

    /// The configuration that was used to configure this 8259 chip.
    configuration: Option<ICWs>,
}

impl Pic {
    /// The End of Interrupt command, used to reply to the PICs at the end of an interrupt handler
    const _EOI: u8 = 0x20;

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
    pub fn get_interrupt_masks(&self) -> u8 {
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
    pub unsafe fn set_interrupt_masks(&mut self, masks: u8) {
        let ocw1 = OCW1::new().set_interrupt_masks(masks);

        self.send_ocw1(ocw1);
    }

    unsafe fn send_ocw1(&mut self, operation_control_word: OCW1) {
        self.data.write(operation_control_word.byte);
    }

    unsafe fn send_ocw2(&mut self, operation_control_word: OCW2) {
        self.command.write(operation_control_word.byte);
    }

    unsafe fn send_ocw3(&mut self, operation_control_word: OCW3) {
        self.command.write(operation_control_word.byte);
    }

    unsafe fn send_icw1(&mut self, initialization_control_word: ICW1) {
        self.command.write(initialization_control_word.byte);
    }

    unsafe fn send_icw2(&mut self, initialization_control_word: ICW2) {
        self.data.write(initialization_control_word.byte);
    }

    unsafe fn send_icw3(&mut self, initialization_control_word: ICW3) {
        self.data.write(initialization_control_word.byte);
    }

    unsafe fn send_icw4(&mut self, initialization_control_word: ICW4) {
        self.data.write(initialization_control_word.byte);
    }

    unsafe fn send_non_specific_eoi(&mut self) {
        let ocw2 = OCW2::new().set_ir_level(0).set_non_specific_eoi();

        self.send_ocw2(ocw2);
    }

    pub fn configure(&mut self, config: ICWs) {
        assert!(config.is_complete());
        // println!("ICW1: {:x}", config.icw1.unwrap().byte);
        // println!("ICW2: {:x}", config.icw2.unwrap().byte);
        // println!("ICW3: {:x}", config.icw3.unwrap().byte);
        // println!("ICW4: {:x}", config.icw4.unwrap().byte);

        let icw1 = config.icw1.unwrap();
        let icw2 = config.icw2.unwrap();
        let icw3 = config.icw3.unwrap();
        let icw4 = config.icw4.unwrap();

        unsafe {
            self.send_icw1(icw1);
            self.send_icw2(icw2);
            self.send_icw3(icw3);
            self.send_icw4(icw4);
        }
        self.configuration = Some(config);
    }

    pub fn get_register(&mut self, register: PICRegister) -> u8 {
        if register == PICRegister::InterruptMasks {
            return self.get_interrupt_masks();
        }

        let ocw3 = OCW3::new().set_read_register(register);

        unsafe {
            self.send_ocw3(ocw3);
        }
        self.command.read()
    }
}

/// Describes the triggering mode of interrupts for the chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TriggeringMode {
    Level,
    Edge,
}

/// Describes the kind of 8259 chip: Master or Slave.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PICKind {
    Master,
    Slave,
}

/// Describes a buffering mode for a 8259 chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BufferingMode {
    NotBuffered,
    SlaveBuffered,
    MasterBuffered,
}

/// Describes a register for a 8259 chip.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PICRegister {
    InRequest,
    InService,
    InterruptMasks,
}

/// This data structure aggregates aggregations (the ICWs date structure) of ICWs,
/// This describes a complete configuration for all the chips being configured.
/// This asserts some conditions to make sure (At least for the obvious cases) that
/// the overall configuration is not malformed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PicConfiguration {
    configurations: [Option<ICWs>; 8],
    nbr_pics: usize,
    master_index: Option<usize>,
}

impl PicConfiguration {
    /// Creates a new, unsetup, PicConfiguration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new ICWs (configuration for a specific 8259 chip)
    /// to the, global, PicConfiguration.
    ///
    /// Panic:
    /// - Panics if the PicConfiguration is full, that is, if already 8 chips
    /// are being configured.
    /// - If the given `config` chip configuration is not complete.
    /// - If adding a `config` for a master chip while there is already one registed
    ///   is attempted.
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

    /// Returns whether a master chip configuration has been registered.
    pub fn has_master(&self) -> bool {
        self.master_index.is_some()
    }

    /// Gets the configuration for the master chip.
    ///
    /// Panic:
    /// Panics if not such configuration has been registered.
    pub fn get_master(&self) -> &ICWs {
        assert!(self.has_master(), "No PIC master were found.");
        self.configurations[self.master_index.unwrap()]
            .as_ref()
            .unwrap()
    }

    /// Returns the number of chip being registered.
    pub fn nbr_pics(&self) -> usize {
        self.nbr_pics
    }

    /// Returns whether the configuration of all registered PICs is complete.
    pub fn is_complete(&self) -> bool {
        return self.nbr_pics != 0
            && self.has_master()
            && self.get_master().cascaded_lines() == self.nbr_pics.checked_sub(1).unwrap_or(0);
    }

    /// Returns an iterator over all the slaves configurations (ICWs).
    pub fn slaves<'a>(&'a self) -> impl Iterator<Item = ICWs> + 'a {
        let mut slaves = self
            .configurations
            .iter()
            .filter_map(|&c| c)
            .filter(|&c| c.pic_kind() == PICKind::Slave);
        unfold((), move |_| slaves.next())
    }
}

/// PIC idt vectors
extern "C" {
    static mut _pic_handlers_array: [u32; 16];
}
