//! See [ISR](https://wiki.osdev.org/ISR)
use crate::ffi::{c_char, strlen};

extern "C" {
    pub(super) fn _isr_timer();
    pub(super) fn _isr_keyboard();
    pub(super) fn _isr_cascade();
    pub(super) fn _isr_com2();
    pub(super) fn _isr_com1();
    pub(super) fn _isr_lpt2();
    pub(super) fn _isr_floppy_disk();
    pub(super) fn _isr_lpt1();
    pub(super) fn _isr_cmos();
    pub(super) fn _isr_acpi();
    pub(super) fn _isr_ps2_mouse();
    pub(super) fn _isr_fpu_coproc();
    pub(super) fn _isr_primary_hard_disk();
    pub(super) fn _isr_secondary_hard_disk();
}

/// For now, this is assigned as the handler for every interrupt that are not exceptions
/// Specifically handling the case for the keyboard, just for testing that it's working.
#[no_mangle]
extern "C" fn generic_interrupt_handler(interrupt_name: *const u8) {
    println!("in interrupt context");
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(interrupt_name, strlen(interrupt_name as *const c_char)) };
    println!("From interrupt: {}", unsafe { core::str::from_utf8_unchecked(slice) })
}

/// This is the handler set to the reserved Gate Entries.
/// Panics when called.
#[no_mangle]
pub(super) extern "C" fn reserved_interruption() {
    panic!("Reserved interruption raised");
}
extern "C" {
    fn _process_a();
    fn _process_b();
}

use crate::system::BaseRegisters;

#[derive(Debug, Copy, Clone)]
struct Process {
    eip: u32,
    registers: BaseRegisters,
}

use alloc::vec;
use alloc::vec::Vec;

// static mut PROCESS: Vec<Process> = vec![
//     Process { eip: _process_a as u32, registers: Default::default() },
//     Process { eip: _process_b as u32, registers: Default::default() },
// ];

static mut CURR_PROCESS: Option<u32> = None;

#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(old_eip: u32, old_esp: u32, registers: BaseRegisters) -> ! {
    eprintln!("bonjour {:?} {:?} {:?}", old_eip, old_esp, registers);
    //let old_eip = *ret_eip;
    // *ret_eip =
    //     if CURR_PROCESS == 0 || CURR_PROCESS == _process_b as u32 { _process_a as u32 } else { _process_b as u32 };
    // CURR_PROCESS = *ret_eip;
    // eprintln!("{:X?}", *registers)
    // eprintln!(" {:X?}", old_eip);
    loop {
        asm!("hlt":::: "volatile");
    }
}

#[no_mangle]
extern "C" fn process_a() {
    eprintln!("process A ");
}

#[no_mangle]
extern "C" fn process_b() {
    eprintln!("process B ");
}
