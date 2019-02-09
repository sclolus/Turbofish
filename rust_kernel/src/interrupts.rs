mod exceptions;
mod irqs;

#[macro_use]
pub mod macros;
pub mod idt;
pub mod pic_8259;
pub mod pit;

pub use idt::*;
pub use pic_8259::*;

/// Enables interrupts system-wide
#[inline(always)]
pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

/// Disable interrupts system-wide
#[inline(always)]
pub unsafe fn disable() {
    asm!("cli" :::: "volatile");
}

/// Get the current interrupts state
pub fn get_interrupts_state() -> bool {
    use crate::registers::Eflags;

    unsafe { Eflags::get_eflags().interrupt_flag() }
}

/// Restore the interrupts state
pub unsafe fn restore_interrupts_state(state: bool) {
    match state {
        true => enable(),
        false => disable(),
    }
}

/// Wrapper to init the Interrupt Descriptor Table.
pub unsafe fn init() {
    disable();

    Idtr::init_idt();

    pic_8259::save_default_imr();
    pic_8259::set_idt_vectors(pic_8259::KERNEL_PIC_MASTER_IDT_VECTOR, pic_8259::KERNEL_PIC_SLAVE_IDT_VECTOR);
    pic_8259::mask_all_interrupts();
    pic_8259::irq_clear_mask(1); // enable only the keyboard.
    pic_8259::irq_clear_mask(2); // slave cascade

    enable();
}
