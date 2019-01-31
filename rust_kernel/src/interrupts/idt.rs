/// See https://wiki.osdev.org/IDT and https://wiki.osdev.org/Interrupts
use super::idt_gate_entry::*;
use core::ffi::c_void;
use core::mem::size_of;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[repr(packed)]
pub struct Idtr {
    pub length: u16,
    pub idt_addr: *mut IdtGateEntry,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterruptTable<'a> {
    entries: &'a mut [IdtGateEntry],
}

extern "C" {
    fn _load_idtr(param: *const Idtr);
    fn _get_idtr(to_fill: *mut Idtr);

    pub fn generic_asm_isr_wrapper();
}

#[no_mangle]
#[inline(never)]
/// Load the `idtr` passed in parameter into the Interrupt descriptor Table Register
pub unsafe extern "C" fn load_idtr(idtr: Idtr) -> Idtr {
    _load_idtr(&idtr as *const Idtr);
    idtr
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
    const DEFAULT_IDTR_LENGTH: u16 = 1024;
    const DEFAULT_IDTR_ADDR: *mut IdtGateEntry = 0x400 as *mut _;

    /// Current default idtr, the address is 0x400, just above the idt bios
    /// and just below the current GDT
    const DEFAULT_IDTR: Idtr = Idtr { length: Idtr::DEFAULT_IDTR_LENGTH - 1, idt_addr: Idtr::DEFAULT_IDTR_ADDR };

    /// Loads the default Idtr
    pub unsafe fn load_default_idtr() -> Idtr {
        load_idtr(Self::DEFAULT_IDTR)
    }

    /// Returns a `&mut [IdtGateEntr]` of the current idt
    unsafe fn idt_gate_entries_slice_mut(&self) -> &mut [IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / size_of::<IdtGateEntry>() as u16) as usize)
    }

    /// Returns a `&[IdtGateEntr]` of the current idt
    #[allow(dead_code)]
    unsafe fn idt_gate_entries_slice(&self) -> &[IdtGateEntry] {
        core::slice::from_raw_parts_mut(self.idt_addr, (self.length / size_of::<IdtGateEntry>() as u16) as usize)
    }

    /// Construct the Interrupt Table from the Idtr.
    pub unsafe fn get_interrupt_table(&self) -> InterruptTable {
        InterruptTable { entries: self.idt_gate_entries_slice_mut() }
    }
}

use super::exceptions::*;
use super::irqs::*;
use GateType::*;
use InterruptTableError::*;

impl<'a> InterruptTable<'a> {
    /// Those are the current default handlers for the exceptions from 0x0 to 0x1F
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

    /// Those are the current default handlers for the IRQs from the PICs 8259
    /// They are mapped from 0x20 to 0x2F
    const DEFAULT_IRQS: [unsafe extern "C" fn(); 16] = [
        _isr_timer,
        _isr_keyboard,
        _isr_cascade,
        _isr_com2,
        _isr_com1,
        _isr_lpt2,
        _isr_floppy_disk,
        _isr_lpt1,
        _isr_cmos,
        _isr_acpi,
        reserved_interruption,
        reserved_interruption,
        _isr_ps2_mouse,
        _isr_fpu_coproc,
        _isr_primary_hard_disk,
        _isr_secondary_hard_disk,
    ];

    /// This loads the default interrupt table:
    /// All exceptions and interrupts from the PIC.
    pub unsafe fn load_default_interrupt_table(&mut self) {
        let mut gate_entry =
            *IdtGateEntry::new().set_storage_segment(false).set_privilege_level(0).set_selector(1 << 3);

        for (index, &(exception, gate_type)) in Self::DEFAULT_EXCEPTIONS.iter().enumerate() {
            gate_entry.set_handler(exception as *const c_void as u32).set_gate_type(gate_type);

            self.set_interrupt_entry(index, &gate_entry).unwrap();
        }

        let offset = Self::DEFAULT_EXCEPTIONS.len();
        for (index, &interrupt_handler) in Self::DEFAULT_IRQS.iter().enumerate() {
            gate_entry.set_handler(interrupt_handler as *const c_void as u32).set_gate_type(InterruptGate32);

            self.set_interrupt_entry(index + offset, &gate_entry).unwrap();
        }
    }

    /// Set the P flag in type_attr to 1
    /// WARNING: This is not an interrupts::enable call
    /// The interrupt is merely enabled if sli() was called
    pub unsafe fn enable_interrupt(&mut self, interrupt: usize) -> Result<(), InterruptTableError> {
        self.entries.get_mut(interrupt).map_or(Err(ErrIndexOutOfBound), |entry| {
            entry.set_present(true);
            Ok(())
        })
    }

    /// Set the P flag in type_attr to 0
    pub unsafe fn disable_interrupt(&mut self, interrupt: usize) -> Result<(), InterruptTableError> {
        self.entries.get_mut(interrupt).map_or(Err(ErrIndexOutOfBound), |entry| {
            entry.set_present(false);
            Ok(())
        })
    }

    /// Sets the P flag in type_attr to 0 for all the Gate Entries
    /// Warning: This is not an interrupts::disable call
    /// The interrupts can still be fired.
    pub unsafe fn disable_all_interrupts(&mut self) {
        for interrupt in 0..self.entries.len() {
            self.disable_interrupt(interrupt).unwrap();
        }
    }

    /// Sets a perticular Gate entry to a specific value.
    pub unsafe fn set_interrupt_entry(
        &mut self,
        interrupt: usize,
        entry: &IdtGateEntry,
    ) -> Result<(), InterruptTableError> {
        self.entries.get_mut(interrupt).map_or(Err(ErrIndexOutOfBound), |slot| {
            *slot = *entry;
            Ok(())
        })
    }

    /// Returns the InterruptTable as a slice of IdtGateEntry.
    pub fn as_slice(&self) -> &[IdtGateEntry] {
        self.entries
    }
}

#[derive(Debug)]
pub enum InterruptTableError {
    ErrIndexOutOfBound,
    UnknownError,
}
