//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_

use super::{CpuState, SysResult, SCHEDULER};

mod mmap;

use errno::Errno;

use core::ffi::c_void;

use crate::interrupts::idt::{GateType::TrapGate32, IdtGateEntry, InterruptTable};
use crate::system::BaseRegisters;

extern "C" {
    fn _isr_syscall();
    fn _sys_test() -> i32;
}

/// Write something into the screen
fn sys_write(fd: i32, buf: *const u8, count: usize) -> SysResult<i32> {
    if fd != 1 {
        Err(Errno::Ebadf)
    } else {
        unsafe {
            asm!("cli");
            eprint!("{}", core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count)));
            asm!("sti");
        }
        Ok(count as i32)
    }
}

/// Read something from a file descriptor
fn sys_read(_fd: i32, _buf: *const u8, _count: usize) -> SysResult<i32> {
    unimplemented!();
}

/// Exit from a process
unsafe fn sys_exit(status: i32) -> ! {
    asm!("cli");
    SCHEDULER.lock().exit(status);
}

/// Fork a process
unsafe fn sys_fork(kernel_esp: u32) -> SysResult<i32> {
    asm!("cli");
    let res = SCHEDULER.lock().fork(kernel_esp);
    asm!("sti");
    res
}

/// Preemptif coherency checker
unsafe fn sys_test() -> SysResult<i32> {
    if _sys_test() == 0 {
        Ok(0)
    } else {
        Err(Errno::Eperm)
    }
}

/// Do a stack overflow on the kernel stack
#[allow(unconditional_recursion)]
unsafe fn sys_stack_overflow(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> SysResult<i32> {
    asm!("cli");
    eprintln!("Stack overflow syscall on the fly: v = {:?}", a + b + c + d + e + f);
    asm!("sti");
    Ok(sys_stack_overflow(a + 1, b + 1, c + 1, d + 1, e + 1, f + 1).unwrap())
}

/// Global syscall interrupt handler called from assembly code
#[no_mangle]
pub unsafe extern "C" fn syscall_interrupt_handler(cpu_state: *mut CpuState) {
    #[allow(unused_variables)]
    let BaseRegisters { eax, ebx, ecx, edx, esi, edi, ebp, .. } = (*cpu_state).registers;

    let result = match eax {
        0x1 => sys_exit(ebx as i32),       // This syscall doesn't return !
        0x2 => sys_fork(cpu_state as u32), // CpuState represents kernel_esp
        0x3 => sys_read(ebx as i32, ecx as *const u8, edx as usize),
        0x4 => sys_write(ebx as i32, ecx as *const u8, edx as usize),
        0x80000000 => sys_test(),
        0x80000001 => sys_stack_overflow(0, 0, 0, 0, 0, 0),
        // set thread area: WTF
        0xf3 => Err(Errno::Eperm),
        sysnum => panic!("wrong syscall {}", sysnum),
    };

    (*cpu_state).registers.eax = match result {
        Ok(return_value) => return_value as u32,
        Err(errno) => (errno as i32 * -1) as u32,
    };
}

/// Initialize all the syscall system by creation of a new IDT entry at 0x80
pub fn init() {
    let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table().unwrap() };

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(3)
        .set_selector(1 << 3)
        .set_gate_type(TrapGate32);
    gate_entry.set_handler(_isr_syscall as *const c_void as u32);
    interrupt_table[0x80] = gate_entry;
}
