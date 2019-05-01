//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_
#[macro_use]
pub mod test_syscall;
pub use test_syscall::*;
mod syscall_isr;

use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use crate::process::scheduler::SCHEDULER;
use core::ffi::c_void;

fn sys_write(_fd: i32, buf: *const u8, count: usize) {
    unsafe {
        println!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count)));
    }
}

fn sys_read(_fd: i32, _buf: *const u8, _count: usize) {
    unimplemented!();
}

fn sys_exit(status: i32) {
    SCHEDULER.lock().exit(status);
}

fn sys_fork() {
    SCHEDULER.lock().fork();
}

#[no_mangle]
pub extern "C" fn syscall_interrupt_handler(args: [u32; 6]) {
    match args[0] {
        0x1 => sys_exit(args[1] as i32),
        0x2 => sys_fork(),
        0x3 => sys_read(args[1] as i32, args[2] as *const u8, args[3] as usize),
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
