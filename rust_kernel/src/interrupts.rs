mod exceptions;
mod irqs;

pub mod idt;
pub mod idt_gate_entry;
pub mod pic_8259;

pub use idt::*;
pub use idt_gate_entry::*;
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

/// Wrapper to init the Interrupt Descriptor Table.
pub unsafe fn init() {
    disable();

    let idt = Idtr::load_default_idtr();
    println!("Current idtr: {:?}", idt);

    idt.get_interrupt_table().load_default_interrupt_table();

    pic_8259::initialize(0x20, 0x28);
    pic_8259::disable_pics();
    pic_8259::irq_clear_mask(1); // enable only the keyboard.

    enable();
}
