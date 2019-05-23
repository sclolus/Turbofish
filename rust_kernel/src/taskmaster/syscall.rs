//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_

use super::{CpuState, SCHEDULER};

mod mmap;

use core::ffi::c_void;

use crate::interrupts::idt::{GateType::InterruptGate32, IdtGateEntry, InterruptTable};
use crate::system::BaseRegisters;

extern "C" {
    fn _isr_syscall();
}

#[derive(Debug, Copy, Clone)]
enum Errno {
    OperationNotPermitted = 1,
    BadFileNumber = 9,
    OutOfMemory = 12,
}

/// SyscallResult is just made to handle module errors
type SyscallResult = core::result::Result<i32, Errno>;

/// Write something into the screen
fn sys_write(fd: i32, buf: *const u8, count: usize) -> SyscallResult {
    if fd != 1 {
        Err(Errno::BadFileNumber)
    } else {
        unsafe {
            eprint!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count)));
        }
        Ok(count as i32)
    }
}

/// Read something from a file descriptor
fn sys_read(_fd: i32, _buf: *const u8, _count: usize) -> SyscallResult {
    unimplemented!();
}

/// Exit from a process
fn sys_exit(_status: i32) -> SyscallResult {
    unimplemented!();
}

/// Fork a process
fn sys_fork(cpu_state: CpuState) -> SyscallResult {
    let ret = SCHEDULER.lock().fork(cpu_state);
    if ret < 0 {
        Err(Errno::OutOfMemory)
    } else {
        Ok(ret)
    }
}

/// Global syscall interrupt handler called from assembly code
#[no_mangle]
pub unsafe extern "C" fn syscall_interrupt_handler(cpu_state: *mut CpuState) {
    #[allow(unused_variables)]
    let BaseRegisters { eax, ebx, ecx, edx, esi, edi, ebp, .. } = (*cpu_state).registers;

    let result = match eax {
        0x1 => sys_exit(ebx as i32),
        0x2 => sys_fork(*cpu_state),
        0x3 => sys_read(ebx as i32, ecx as *const u8, edx as usize),
        0x4 => sys_write(ebx as i32, ecx as *const u8, edx as usize),
        // set thread area: WTF
        0xf3 => Err(Errno::OperationNotPermitted),
        sysnum => panic!("wrong syscall {}", sysnum),
    };

    (*cpu_state).registers.eax = match result {
        Ok(return_value) => return_value as u32,
        Err(errno) => ((errno as i32) * -1) as u32,
    };
}

/// Initialize all the syscall system by creation of a new IDT entry at 0x80
pub fn init() {
    let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table().unwrap() };

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(3)
        .set_selector(1 << 3)
        .set_gate_type(InterruptGate32);
    gate_entry.set_handler(_isr_syscall as *const c_void as u32);
    interrupt_table[0x80] = gate_entry;
}
