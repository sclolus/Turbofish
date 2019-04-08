use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use core::ffi::c_void;
mod test_syscall;
pub use test_syscall::_write;
mod syscall_isr;

fn sys_write(_fd: i32, buf: *const u8, count: usize) {
    unsafe {
        println!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count)));
    }
}

#[no_mangle]
pub extern "C" fn syscall_interrupt_handler(args: [u32; 6]) {
    match args[0] {
        0x4 => sys_write(args[1] as i32, args[2] as *const u8, args[3] as usize),
        _ => panic!("wrong syscall"),
    }
}

pub fn init() {
    let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table().unwrap() };

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(0)
        .set_selector(1 << 3)
        .set_gate_type(InterruptGate32);
    gate_entry.set_gate_type(InterruptGate32);
    gate_entry.set_handler(syscall_isr::_isr_syscall as *const c_void as u32);
    interrupt_table[0x80] = gate_entry;
}
