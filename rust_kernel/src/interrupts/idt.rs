/// See https://wiki.osdev.org/IDT and https://wiki.osdev.org/Interrupts
mod cpu_exceptions;
mod irqs;
use crate::interrupts::pic_8259;
use bit_field::BitField;
use core::ffi::c_void;
use cpu_exceptions::*;
use irqs::*;

pub type InterruptHandler = extern "C" fn() -> !;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GateType {
    TaskGate32 = 0x5,
    InterruptGate16 = 0x6,
    TrapGate16 = 0x7,
    InterruptGate32 = 0xE,
    TrapGate32 = 0xF,
}

use GateType::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C, packed)]
pub struct IdtGateEntry {
    /// offset bits 0..15. the low part of the address
    pub offset_1: u16,

    /// a code segment selector in GDT or LDT
    pub selector: u16,

    /// unused, set to 0
    pub _zero: u8,

    /// The type attr is layout in this way.
    ///   7                           0
    /// +---+---+---+---+---+---+---+---+
    /// | P |  DPL  | S |    GateType   |
    /// +---+---+---+---+---+---+---+---+

    /// P        	Present	Set to 0 for unused interrupts.
    /// DPL          Descriptor Privilege Level	Gate call protection.
    ///              Specifies which privilege Level the calling Descriptor minimum
    ///              should have.
    ///              So hardware and CPU interrupts can be protected from
    ///              being called out of userspace.
    /// S            Storage Segment	Set to 0 for interrupt and trap gates
    /// Gate Type 	Possible IDT gate types :
    ///              0b0101	0x5	5	80386 32 bit task gate
    ///              0b0110	0x6	6	80286 16-bit interrupt gate
    ///              0b0111	0x7	7	80286 16-bit trap gate
    ///              0b1110	0xE	14	80386 32-bit interrupt gate
    ///              0b1111	0xF	15	80386 32-bit trap gate
    /// type and attributes,
    pub type_attr: u8,

    /// offset bits 16..31
    pub offset_2: u16,
}

impl IdtGateEntry {
    fn minimal() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn new() -> Self {
        let mut new = Self::minimal();

        new.set_present(true);
        new
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.type_attr.set_bit(7, present);
        self
    }

    pub fn set_storage_segment(&mut self, storage: bool) -> &mut Self {
        self.type_attr.set_bit(4, storage);
        self
    }

    pub fn set_gate_type(&mut self, gate_type: GateType) -> &mut Self {
        self.type_attr.set_bits(0..4, gate_type as u8);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u8) -> &mut Self {
        self.type_attr.set_bits(4..6, dpl);
        self
    }

    pub fn set_selector(&mut self, selector: u16) -> &mut Self {
        self.selector = selector;
        self
    }

    pub fn set_handler(&mut self, handler: u32) -> &mut Self {
        self.offset_1 = handler as u16;
        self.offset_2 = ((handler as usize) >> 16) as u16;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[repr(packed)]
pub struct Idtr {
    pub length: u16,
    pub idt_addr: *mut IdtGateEntry,
}

extern "C" {
    fn _load_idtr(param: *const Idtr);
    fn _get_idtr(to_fill: *mut Idtr);

    pub fn generic_asm_isr_wrapper();
}

#[no_mangle]
#[inline(never)]
/// Returns the current Interrupt Descriptor Table Register
pub unsafe extern "C" fn get_idtr() -> Idtr {
    // Temporary struct Idtr to be filled by the asm routine
    let mut idtr = Idtr { length: 0, idt_addr: 1 as *mut _ };

    _get_idtr(&mut idtr as *mut _);
    idtr
}

impl Idtr {
    const DEFAULT_IDTR_SIZE: u16 = 256 * 8;
    const DEFAULT_IDTR_ADDR: *mut IdtGateEntry = 0x1000 as *mut _;

    const DEFAULT_EXCEPTIONS: [(unsafe extern "C" fn() -> !, GateType); 32] = [
        (_isr_divide_by_zero, InterruptGate32),
        (_isr_debug, TrapGate32),
        (_isr_non_maskable_interrupt, InterruptGate32),
        (_isr_breakpoint, TrapGate32),
        (_isr_overflow, TrapGate32),
        (_isr_bound_range_exceeded, InterruptGate32),
        (_isr_invalid_opcode, InterruptGate32),
        (_isr_no_device, InterruptGate32),
        (_isr_double_fault, InterruptGate32),
        (_isr_fpu_seg_overrun, InterruptGate32),
        (_isr_invalid_tss, InterruptGate32),
        (_isr_seg_no_present, InterruptGate32),
        (_isr_stack_seg_fault, InterruptGate32),
        (_isr_general_protect_fault, InterruptGate32),
        (_isr_page_fault, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (_isr_fpu_floating_point_exep, InterruptGate32),
        (_isr_alignment_check, InterruptGate32),
        (_isr_machine_check, InterruptGate32),
        (_isr_simd_fpu_fp_exception, InterruptGate32),
        (_isr_virtualize_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
        (_isr_security_exception, InterruptGate32),
        (reserved_exception, InterruptGate32),
    ];

    /// Those are the current default handlers for the IRQs from the PICs 8259 (master)
    /// They are mapped from 0x20 to 0x27
    const DEFAULT_IRQS_MASTER: [unsafe extern "C" fn(); 8] =
        [_isr_timer, _isr_keyboard, _isr_cascade, _isr_com2, _isr_com1, _isr_lpt2, _isr_floppy_disk, _default_isr];

    /// Those are the current default handlers for the IRQs from the PICs 8259 (slave)
    /// They are mapped from 0x70 to 0x77
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

    pub fn init_idt() {
        let idt: Idtr = Idtr { length: Idtr::DEFAULT_IDTR_SIZE - 1, idt_addr: Idtr::DEFAULT_IDTR_ADDR };
        let idt_slice =
            unsafe { core::slice::from_raw_parts_mut(idt.idt_addr, (Idtr::DEFAULT_IDTR_SIZE / 8) as usize) };
        for entry in idt_slice.iter_mut() {
            *entry = *IdtGateEntry::new()
                .set_storage_segment(false)
                .set_privilege_level(0)
                .set_selector(1 << 3)
                .set_gate_type(InterruptGate32)
                .set_handler(_default_isr as *const c_void as u32);
        }

        let mut gate_entry =
            *IdtGateEntry::new().set_storage_segment(false).set_privilege_level(0).set_selector(1 << 3);

        for (index, &(exception, gate_type)) in Self::DEFAULT_EXCEPTIONS.iter().enumerate() {
            gate_entry.set_handler(exception as *const c_void as u32).set_gate_type(gate_type);

            idt_slice[index] = gate_entry;
        }

        let offset = pic_8259::KERNEL_PIC_MASTER_IDT_VECTOR;
        for (index, &interrupt_handler) in Self::DEFAULT_IRQS_MASTER.iter().enumerate() {
            gate_entry.set_handler(interrupt_handler as *const c_void as u32).set_gate_type(InterruptGate32);

            idt_slice[index + offset as usize] = gate_entry;
        }

        let offset = pic_8259::KERNEL_PIC_SLAVE_IDT_VECTOR;
        for (index, &interrupt_handler) in Self::DEFAULT_IRQS_SLAVE.iter().enumerate() {
            gate_entry.set_handler(interrupt_handler as *const c_void as u32).set_gate_type(InterruptGate32);

            idt_slice[index + offset as usize] = gate_entry;
        }
        unsafe {
            _load_idtr(&idt as *const Idtr);
        }
    }
}
