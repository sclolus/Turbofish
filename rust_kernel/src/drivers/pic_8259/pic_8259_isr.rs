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
    fn _switch_process(eflags: u32, segment: u32, eip: u32, esp: u32, registers: BaseRegisters) -> !;
    static process_a_stack: u8;
    static process_b_stack: u8;
    static kernel_stack: u8;
}

use crate::system::BaseRegisters;

#[derive(Debug, Copy, Clone)]
struct Process {
    eip: u32,
    esp: u32,
    eflags: Eflags,
    segment: u32,
    registers: BaseRegisters,
}

use crate::registers::Eflags;
use alloc::vec;
use alloc::vec::Vec;

static mut PROCESS: Option<Vec<Process>> = None;
static mut CURR_PROCESS_INDEX: Option<usize> = None;

unsafe fn switch_process(next_process_index: usize) -> ! {
    let next_process: &Process = &PROCESS.as_mut().unwrap()[next_process_index];
    eprintln!("switch to eip:{:X?} esp:{:X?} reg:{:X?}", next_process.eip, next_process.esp, next_process.registers);
    CURR_PROCESS_INDEX = Some(next_process_index);
    _switch_process(
        next_process.eflags.inner(),
        next_process.segment,
        next_process.eip,
        next_process.esp,
        next_process.registers,
    );
}

#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(
    old_eip: u32,
    old_segment: u32,
    old_eflags: u32,
    old_esp: u32,
    registers: BaseRegisters,
) -> ! {
    // eprintln!("kernel_stack: {:?}\n", &kernel_stack as *const u8);
    let eflags = crate::registers::Eflags::new(old_eflags);
    // eprintln!("bonjour segment: {:X}, eflags: {}", old_segment, eflags);

    eprintln!("sched");
    eprintln!("bonjour eip:{:X?} esp:{:X?} reg:{:X?}", old_eip, old_esp, registers);
    if !PROCESS.is_some() {
        PROCESS = Some(vec![
            Process {
                eip: _process_a as u32,
                esp: symbol_addr!(process_a_stack) as u32,
                registers: Default::default(),
                segment: old_segment,
                eflags: Eflags::new(old_eflags),
            },
            Process {
                eip: _process_b as u32,
                esp: symbol_addr!(process_b_stack) as u32,
                registers: Default::default(),
                segment: old_segment,
                eflags: Eflags::new(old_eflags),
            },
        ]);
        // eprintln!("{:#X?}", PROCESS);
    }
    match CURR_PROCESS_INDEX {
        None => {
            CURR_PROCESS_INDEX = Some(0);
            return switch_process(0);
        }
        Some(i) => {
            let next_process = if i == 0 { 1 } else { 0 };
            let curr_process: &mut Process = &mut PROCESS.as_mut().unwrap()[i];
            curr_process.eip = old_eip;
            curr_process.esp = old_esp;
            curr_process.registers = registers;
            curr_process.eflags = Eflags::new(old_eflags);
            curr_process.segment = old_segment;
            return switch_process(next_process);
        }
    }
}

#[no_mangle]
extern "C" fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn debug_process(eip: u32) {
    eprintln!("jump to eip |{:X}|\n", eip);
    crate::tests::helpers::exit_qemu(0);
    loop {}
}
