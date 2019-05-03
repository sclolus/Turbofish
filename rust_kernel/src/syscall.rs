//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_
#[macro_use]
pub mod test_syscall;
pub use test_syscall::*;
mod mmap;
mod syscall_isr;

use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use crate::process::scheduler::SCHEDULER;
use crate::process::CpuState;
use crate::system::BaseRegisters;
use core::ffi::c_void;

fn sys_write(_fd: i32, buf: *const u8, count: usize) -> i32 {
    unsafe {
        eprint!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count)));
    }
    count as i32
}

fn sys_read(_fd: i32, _buf: *const u8, _count: usize) -> i32 {
    unimplemented!();
}

fn sys_exit(status: i32) -> i32 {
    SCHEDULER.lock().exit(status);
}

fn sys_fork() -> i32 {
    SCHEDULER.lock().fork()
}

#[no_mangle]
pub extern "C" fn syscall_interrupt_handler(cpu_state: CpuState) {
    SCHEDULER.lock().save_process_state(cpu_state);
    let BaseRegisters { eax, ebx, ecx, edx, .. } = cpu_state.registers;
    let return_value = match eax {
        0x1 => sys_exit(ebx as i32),
        0x2 => sys_fork(),
        0x3 => sys_read(ebx as i32, ecx as *const u8, edx as usize),
        0x4 => sys_write(ebx as i32, ecx as *const u8, edx as usize),
        _ => panic!("wrong syscall"),
    };
    SCHEDULER.lock().return_from_syscall(return_value);
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
