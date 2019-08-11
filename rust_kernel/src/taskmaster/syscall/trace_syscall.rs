use libc_binding::{
    CLONE, CLOSE, EXECVE, EXIT, EXIT_QEMU, FORK, GETPGID, GETPGRP, GETPID, GETPPID, KILL, MMAP,
    MPROTECT, MUNMAP, NANOSLEEP, PAUSE, READ, REBOOT, SETPGID, SHUTDOWN, SIGACTION, SIGNAL,
    SIGPROCMASK, SIGRETURN, SIGSUSPEND, SOCKETCALL, STACK_OVERFLOW, TCGETATTR, TCGETPGRP,
    TCSETATTR, TCSETPGRP, TEST, UNLINK, WAITPID, WRITE,
};

use super::mmap::{MmapArgStruct, MmapProt};
use super::nanosleep::TimeSpec;
use super::process::CpuState;
use super::signal::{sigset_t, StructSigaction};
use super::socket::SocketArgsPtr;
use crate::ffi::c_char;
use crate::memory::tools::address::Virt;
use crate::system::BaseRegisters;
use core::ffi::c_void;
use libc_binding::termios;
use libc_binding::Pid;

#[allow(dead_code)]
pub fn trace_syscall(cpu_state: *mut CpuState) {
    let BaseRegisters {
        eax,
        ebx,
        ecx,
        edx,
        // esi,
        // edi,
        // ebp,
        ..
    } = unsafe { (*cpu_state).registers };
    match eax {
        EXIT => eprintln!("exit({:#?})", ebx as i32),
        FORK => eprintln!("fork()"),
        READ => eprintln!(
            "read({:#?}, {:#?}, {:#?})",
            ebx as i32, ecx as *mut u8, edx as usize
        ),
        WRITE => eprintln!(
            "write({:#?}, {:#?}, {:#?})",
            ebx as i32, ecx as *const u8, edx as usize
        ),
        CLOSE => eprintln!("close({:#?})", ebx as i32),
        WAITPID => eprintln!(
            "waitpid({:#?}, {:#?}, {:#?})",
            ebx as i32, ecx as *mut i32, edx as i32
        ),
        UNLINK => eprintln!("unlink({:#?})", ebx as *const u8),
        EXECVE => eprintln!(
            "execve({:#?}, {:#?}, {:#?})",
            ebx as *const c_char, ecx as *const *const c_char, edx as *const *const c_char,
        ),
        GETPID => eprintln!("getpid()"),
        // GETUID             // => eprintln!("getuid({:#?})",), TODO: need to be implemented
        PAUSE => eprintln!("pause()"),
        KILL => eprintln!("kill({:#?}, {:#?})", ebx as i32, ecx as u32),
        SIGNAL => eprintln!("signal({:#?}, {:#?})", ebx as u32, ecx as usize),
        SETPGID => eprintln!("setpgid({:#?}, {:#?})", ebx as Pid, ecx as Pid),
        GETPPID => eprintln!("getppid()"),
        GETPGRP => eprintln!("getpgrp()"),
        SIGACTION => eprintln!(
            "sigaction({:#?}, {:#?}, {:#?})",
            ebx as u32, ecx as *const StructSigaction, edx as *mut StructSigaction,
        ),
        SIGSUSPEND => eprintln!("sigsuspend({:#?})", ebx as *const sigset_t),
        REBOOT => eprintln!("reboot()"),
        MMAP => eprintln!("mmap({:#?})", ebx as *const MmapArgStruct),
        MUNMAP => eprintln!("munmap({:#?}, {:#?})", Virt(ebx as usize), ecx as usize),
        SOCKETCALL => eprintln!("socketcall({:#?}, {:#?})", ebx as u32, ecx as SocketArgsPtr),
        CLONE => eprintln!(
            "clone({:#?}, {:#?}, {:#?})",
            cpu_state as u32, ebx as *const c_void, ecx as u32
        ),
        MPROTECT => eprintln!(
            "mprotect({:#?}, {:#?}, {:#?})",
            Virt(ebx as usize),
            ecx as usize,
            MmapProt::from_bits_truncate(edx),
        ),
        SIGPROCMASK => eprintln!(
            "sigprocmask({:#?}, {:#?}, {:#?})",
            ebx as i32, ecx as *const sigset_t, edx as *mut sigset_t
        ),
        GETPGID => eprintln!("getpgid({:#?})", ebx as Pid),
        NANOSLEEP => eprintln!(
            "nanosleep({:#?}, {:#?})",
            ebx as *const TimeSpec, ecx as *mut TimeSpec
        ),
        SIGRETURN => eprintln!("sigreturn({:#?})", cpu_state),
        SHUTDOWN => eprintln!("shutdown()"),
        TEST => eprintln!("test()"),
        STACK_OVERFLOW => eprintln!("stack_overflow()"),
        EXIT_QEMU => eprintln!("exit_qemu({:#?})", ebx as u32),
        TCGETATTR => eprintln!("tcgetattr({:#?}, {:#?})", ebx as i32, ecx as *mut termios),
        TCSETATTR => eprintln!(
            "tcsetattr({:#?}, {:#?}, {:#?})",
            ebx as i32, ecx as u32, edx as *const termios
        ),
        TCSETPGRP => eprintln!("tcsetpgrp({:#?}, {:#?})", ebx as i32, ecx as Pid),
        TCGETPGRP => eprintln!("tcgetpgrp({:#?})", ebx as i32),
        _ => eprintln!("unknown syscall()",),
    }
}
