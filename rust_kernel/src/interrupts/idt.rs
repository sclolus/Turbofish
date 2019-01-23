use super::idt_gate_entry::*;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[repr(packed)]
pub struct Idtr {
    pub length: u16,
    pub idt_addr: *mut IdtGateEntry,
}

pub struct InterruptTable<'a> {
    entries: &'a mut [IdtGateEntry],
}

extern "C" {
    fn asm_load_idtr(param: *const Idtr);
    fn asm_get_idtr(to_fill: *mut Idtr);
    fn asm_int(int: u32) -> ();

    pub fn generic_asm_isr_wrapper();
}

#[no_mangle]
#[inline(never)]
// Load the idtr passed in parameter into the Interrupt descriptor Table Register
pub extern "C" fn _load_idtr(idtr: Idtr) -> Idtr {
    unsafe {
        asm_load_idtr(&idtr as *const Idtr);
        idtr
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _asm_int(int: u32) -> () {
    unsafe {
        asm_int(int);
    }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _get_idtr() -> Idtr {
    unsafe {
        // Temporary struct Idtr to be filled by the asm routine
        let mut idtr = Idtr { length: 0, idt_addr: 1 as *mut _ };

        asm_get_idtr(&mut idtr as *mut _);
        idtr
    }
}

impl Idtr {
    const DEFAULT_IDTR_LENGTH: u16 = 1024;
    const DEFAULT_IDTR_ADDR: *mut IdtGateEntry = 0x400 as *mut _;

    // Current default idtr, the address is 0x400, just above the idt bios
    // and just below the current GDT
    const DEFAULT_IDTR: Idtr = Idtr { length: Idtr::DEFAULT_IDTR_LENGTH, idt_addr: Idtr::DEFAULT_IDTR_ADDR };

    // Loads the default idtr
    pub unsafe fn load_default_idtr() -> Idtr {
        _load_idtr(Self::DEFAULT_IDTR)
    }

    // Primitives to get a slice of the IDT entries.
    unsafe fn idt_gate_entries_slice_mut(&self) -> &mut [IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    unsafe fn idt_gate_entries_slice(&self) -> &[IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / 8) as usize)
    }

    // Construct the Interrupt Table.
    pub unsafe fn get_interrupt_table(&self) -> InterruptTable {
        InterruptTable { entries: self.idt_gate_entries_slice_mut() }
    }
}

use super::exceptions::*;
use GateType::*;

impl<'a> InterruptTable<'a> {
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

    // This loads the default interrupt table:
    // All exceptions and interrupts from the PIC.
    pub fn load_default_interrupt_table(&mut self) {
        let mut gate_entry = *IdtGateEntry::new()
            .set_storage_segment(false)
            .set_privilege_level(0)
            .set_selector(1 << 3);

        for (index, &(exception, gate_type)) in Self::DEFAULT_EXCEPTIONS.iter().enumerate() {
            gate_entry.set_handler(exception as *const u32 as u32)
                .set_gate_type(gate_type);

            self.set_interrupt_entry(index, &gate_entry);
        }

        // for index in 32..=127 {
        //     gate_entry.set_handler(super::interrupts::_isr_keyboard as *const u32 as u32)
        //     .set_gate_type(InterruptGate32);
        //     self.set_interrupt_entry(index, &gate_entry);
        // }
    }

    // Set the P flag in type_attr to 1
    // WARNING: This is not an sli call
    // The interrupt is merely enabled if sli() was called
    pub fn enable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].set_present(true);
    }

    // Set the P flag in type_attr to 1
    pub fn disable_interrupt(&mut self, interrupt: usize) {
        self.entries[interrupt].set_present(false);
    }

    // Sets the P flag in type_attr to 0 for all the Gate Entries
    // Warning: This is not an cli call
    // The interrupts can still be fired.
    pub fn disable_all_interrupts(&mut self) {
        for interrupt in 0..self.entries.len() {
            self.disable_interrupt(interrupt);
        }
    }

    // Sets a perticular Gate entry to a specific value.
    pub fn set_interrupt_entry(&mut self, interrupt: usize, entry: &IdtGateEntry) {
        self.entries[interrupt] = *entry;
    }
}
