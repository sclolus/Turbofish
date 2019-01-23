mod exceptions;
mod interrupts;

pub mod idt_gate_entry;
pub mod idt;
pub mod pic_8259;

pub use idt_gate_entry::*;
pub use idt::*;
pub use pic_8259::*;

#[inline(always)]
pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

#[inline(always)]
pub unsafe fn disable() {
    asm!("cli" :::: "volatile");
}
