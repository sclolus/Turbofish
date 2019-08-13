//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_

use super::process;
use super::process::CpuState;
use super::safe_ffi;
use super::scheduler;
use super::scheduler::{Pid, SCHEDULER};
use super::signal;
use super::signal::{sigset_t, StructSigaction};
use super::task;
use super::{IntoRawResult, SysResult};
use crate::ffi::c_char;
use crate::interrupts::idt::{GateType, IdtGateEntry, InterruptTable};
use crate::memory::tools::address::Virt;
use crate::system::BaseRegisters;
use libc_binding::{
    termios, CLONE, CLOSE, EXECVE, EXIT, EXIT_QEMU, FORK, GETPGID, GETPGRP, GETPID, GETPPID, KILL,
    MMAP, MPROTECT, MUNMAP, NANOSLEEP, PAUSE, READ, REBOOT, SETPGID, SHUTDOWN, SIGACTION, SIGNAL,
    SIGPROCMASK, SIGRETURN, SIGSUSPEND, SOCKETCALL, STACK_OVERFLOW, TCGETATTR, TCGETPGRP,
    TCSETATTR, TCSETPGRP, TEST, UNLINK, WAITPID, WRITE,
};

use core::ffi::c_void;
use errno::Errno;

mod mmap;
use mmap::{sys_mmap, MmapArgStruct};

mod nanosleep;
use nanosleep::{sys_nanosleep, TimeSpec};

mod waitpid;
use waitpid::sys_waitpid;

pub mod signalfn;
use signalfn::{
    sys_kill, sys_pause, sys_sigaction, sys_signal, sys_sigprocmask, sys_sigreturn, sys_sigsuspend,
};

mod close;
use close::sys_close;

mod unlink;
use unlink::sys_unlink;

mod socket;
use socket::{sys_socketcall, SocketArgsPtr};

pub mod read;
use read::sys_read;

mod execve;
use execve::sys_execve;

pub mod clone;
use clone::sys_clone;

mod tcsetattr;
use tcsetattr::sys_tcsetattr;

mod tcgetattr;
use tcgetattr::sys_tcgetattr;

mod tcsetpgrp;
use tcsetpgrp::sys_tcsetpgrp;

mod tcgetpgrp;
use tcgetpgrp::sys_tcgetpgrp;

mod write;
use write::sys_write;

mod getpid;
use getpid::sys_getpid;

mod getppid;
use getppid::sys_getppid;

mod exit;
use exit::sys_exit;

mod mprotect;
use mprotect::{sys_mprotect, MmapProt};

mod munmap;
use munmap::sys_munmap;

mod reboot;
use reboot::sys_reboot;

mod shutdown;
use shutdown::sys_shutdown;

mod stack_overflow;
use stack_overflow::sys_stack_overflow;

mod test;
use test::sys_test;

mod fork;
use fork::sys_fork;

mod getpgrp;
use getpgrp::sys_getpgrp;

mod getpgid;
use getpgid::sys_getpgid;

mod setpgid;
use setpgid::sys_setpgid;

mod trace_syscall;

extern "C" {
    fn _isr_syscall();

    fn _get_pit_time() -> u32;
    fn _get_process_end_time() -> u32;
}

/// Global syscall interrupt handler called from assembly code
/// See https://www.informatik.htw-dresden.de/~beck/ASM/syscall_list.html
#[no_mangle]
pub unsafe extern "C" fn syscall_interrupt_handler(cpu_state: *mut CpuState) {
    #[allow(unused_variables)]
    let BaseRegisters {
        eax,
        ebx,
        ecx,
        edx,
        esi,
        edi,
        ebp,
        ..
    } = (*cpu_state).registers;

    // trace_syscall::trace_syscall(cpu_state);
    let result = match eax {
        EXIT => sys_exit(ebx as i32),       // This syscall doesn't return !
        FORK => sys_fork(cpu_state as u32), // CpuState represents kernel_esp
        READ => sys_read(ebx as i32, ecx as *mut u8, edx as usize),
        WRITE => sys_write(ebx as i32, ecx as *const u8, edx as usize),
        CLOSE => sys_close(ebx as i32),
        WAITPID => sys_waitpid(ebx as i32, ecx as *mut i32, edx as i32),
        UNLINK => sys_unlink(ebx as *const u8),
        EXECVE => sys_execve(
            ebx as *const c_char,
            ecx as *const *const c_char,
            edx as *const *const c_char,
        ),
        GETPID => sys_getpid(),
        // GETUID             //        // => sys_getuid(), TODO: need to be implemented
        PAUSE => sys_pause(),
        KILL => sys_kill(ebx as i32, ecx as u32),
        SIGNAL => sys_signal(ebx as u32, ecx as usize),
        SETPGID => sys_setpgid(ebx as Pid, ecx as Pid),
        GETPPID => sys_getppid(),
        GETPGRP => sys_getpgrp(),
        SIGACTION => sys_sigaction(
            ebx as u32,
            ecx as *const StructSigaction,
            edx as *mut StructSigaction,
        ),
        SIGSUSPEND => sys_sigsuspend(ebx as *const sigset_t),
        REBOOT => sys_reboot(),
        MMAP => sys_mmap(ebx as *const MmapArgStruct),
        MUNMAP => sys_munmap(Virt(ebx as usize), ecx as usize),
        SOCKETCALL => sys_socketcall(ebx as u32, ecx as SocketArgsPtr),
        CLONE => sys_clone(cpu_state as u32, ebx as *const c_void, ecx as u32),
        MPROTECT => sys_mprotect(
            Virt(ebx as usize),
            ecx as usize,
            MmapProt::from_bits_truncate(edx),
        ),
        SIGPROCMASK => sys_sigprocmask(ebx as i32, ecx as *const sigset_t, edx as *mut sigset_t),
        GETPGID => sys_getpgid(ebx as Pid),
        NANOSLEEP => sys_nanosleep(ebx as *const TimeSpec, ecx as *mut TimeSpec),
        SIGRETURN => sys_sigreturn(cpu_state),
        SHUTDOWN => sys_shutdown(),
        TEST => sys_test(),
        STACK_OVERFLOW => sys_stack_overflow(0, 0, 0, 0, 0, 0),
        EXIT_QEMU => crate::tests::helpers::exit_qemu(ebx as u32),
        TCGETATTR => sys_tcgetattr(ebx as i32, ecx as *mut termios),
        TCSETATTR => sys_tcsetattr(ebx as i32, ecx as u32, edx as *const termios),
        TCSETPGRP => sys_tcsetpgrp(ebx as i32, ecx as Pid),
        TCGETPGRP => sys_tcgetpgrp(ebx as i32),

        // set thread area: WTF
        0xf3 => Err(Errno::Eperm),
        sysnum => panic!("wrong syscall {}", sysnum),
    };

    // trace_syscall::trace_syscall_result(cpu_state, result);

    let is_in_blocked_syscall = result == Err(Errno::Eintr);
    // Note: do not erase eax if we've just been interrupted from a blocked syscall as we must keep
    // the syscall number contained in eax, in case of SA_RESTART behavior
    if is_in_blocked_syscall == false {
        // Return value will be on EAX. Errno always represents the low 7 bits
        (*cpu_state).registers.eax = result.into_raw_result();
    }
    // If ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame. UNLOCK interruptible().
    // If ring0 process -> Can't happened normally
    unpreemptible_context! {{
        SCHEDULER.lock().current_task_deliver_pending_signals(cpu_state, is_in_blocked_syscall);
    }}
}

extern "C" {
    fn _schedule_next();
}

/// Initialize all the syscall system by creation of a new IDT entry at 0x80
pub fn init() {
    let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table().unwrap() };

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(3)
        .set_selector(1 << 3)
        .set_gate_type(GateType::TrapGate32);
    gate_entry.set_handler(_isr_syscall as *const c_void as u32);
    interrupt_table[0x80] = gate_entry;
}
