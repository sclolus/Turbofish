use libc_binding::{
    CLONE, CLOSE, DUP, DUP2, EXECVE, EXIT, EXIT_QEMU, FCNTL, FORK, GETEGID, GETEUID, GETGID,
    GETGROUPS, GETPGID, GETPGRP, GETPID, GETPPID, GETUID, ISATTY, KILL, LSEEK, MMAP, MPROTECT,
    MUNMAP, NANOSLEEP, OPEN, PAUSE, PIPE, READ, REBOOT, SETEGID, SETEUID, SETGID, SETGROUPS,
    SETPGID, SETUID, SHUTDOWN, SIGACTION, SIGNAL, SIGPROCMASK, SIGRETURN, SIGSUSPEND, SOCKETCALL,
    STACK_OVERFLOW, TCGETATTR, TCGETPGRP, TCSETATTR, TCSETPGRP, TEST, UNLINK, WAITPID, WRITE,
};

use super::mmap::MmapArgStruct;
use super::nanosleep::TimeSpec;
use super::process::CpuState;
use super::signal_interface::{sigset_t, StructSigaction};
use super::Fd;
use super::MmapProt;
use super::SocketArgsPtr;
use super::SysResult;
use crate::ffi::c_char;
use crate::memory::tools::address::Virt;
use crate::system::BaseRegisters;
use core::ffi::c_void;
use libc_binding::{gid_t, termios, uid_t, Pid};

#[allow(dead_code)]
pub fn trace_syscall(cpu_state: *mut CpuState) {
    let BaseRegisters {
        eax,
        ebx,
        ecx,
        edx,
        esi,
        // edi,
        // ebp,
        ..
    } = unsafe { (*cpu_state).registers };
    unpreemptible_context!({
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
            // TODO: type parameter are not set and manage the third argument
            OPEN => eprintln!("open({:#?}, {:#?})", ebx as *const u8, ecx as u32),
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
            LSEEK => eprintln!(
                "lseek({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as u64 + ((edx as u64) << 32),
                esi as i32
            ),
            GETPID => eprintln!("getpid()"),
            SETUID => eprintln!("setuid({:#?})", ebx as uid_t),
            GETUID => eprintln!("getuid()"),
            PAUSE => eprintln!("pause()"),
            KILL => eprintln!("kill({:#?}, {:#?})", ebx as i32, ecx as u32),
            PIPE => eprintln!("pipe({:#?})", ebx as *const i32),
            DUP => eprintln!("dup({:#?})", ebx as u32),
            SETGID => eprintln!("setgid({:#?})", ebx as gid_t),
            GETGID => eprintln!("getgid()"),
            GETEUID => eprintln!("geteuid()"),
            FCNTL => eprintln!(
                "fcntl({:#?}, {:#?}, {:#?})",
                ebx as Fd, ecx as u32, edx as Fd
            ),
            GETEGID => eprintln!("getegid()"),
            SIGNAL => eprintln!("signal({:#?}, {:#?})", ebx as u32, ecx as usize),
            SETPGID => eprintln!("setpgid({:#?}, {:#?})", ebx as Pid, ecx as Pid),
            GETPPID => eprintln!("getppid()"),
            DUP2 => eprintln!("dup2({:#?}, {:#?})", ebx as u32, ecx as u32),
            GETPGRP => eprintln!("getpgrp()"),
            SIGACTION => eprintln!(
                "sigaction({:#?}, {:#?}, {:#?})",
                ebx as u32, ecx as *const StructSigaction, edx as *mut StructSigaction,
            ),
            SIGSUSPEND => eprintln!("sigsuspend({:#?})", ebx as *const sigset_t),
            GETGROUPS => eprintln!("getgroups({:#?}, {:#?})", ebx as i32, ecx as *mut gid_t),
            SETGROUPS => eprintln!("setgroups({:#?}, {:#?})", ebx as i32, ecx as *const gid_t),
            REBOOT => eprintln!("reboot()"),
            MMAP => unsafe { eprintln!("mmap({:#?})", *(ebx as *const MmapArgStruct)) },
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
            SETEGID => eprintln!("setegid({:#?})", ebx as gid_t),
            SETEUID => eprintln!("seteuid({:#?})", ebx as uid_t),
            ISATTY => eprintln!("isatty({:#?})", ebx as u32),
            _ => eprintln!("unknown syscall()",),
        }
    })
}

#[allow(dead_code)]
pub fn trace_syscall_result(cpu_state: *mut CpuState, result: SysResult<u32>) {
    let BaseRegisters { eax, .. } = unsafe { (*cpu_state).registers };

    let sysname = match eax {
        EXIT => "exit",
        FORK => "fork",
        READ => "read",
        WRITE => "write",
        CLOSE => "close",
        WAITPID => "waitpid",
        UNLINK => "unlink",
        EXECVE => "execve",
        LSEEK => "lseek",
        GETPID => "getpid",
        SETUID => "setuid",
        GETUID => "getuid",
        PAUSE => "pause",
        KILL => "kill",
        PIPE => "pipe",
        GETGID => "getgid",
        GETEUID => "geteuid",
        FCNTL => "fcntl",
        GETEGID => "getegid",
        SIGNAL => "signal",
        SETPGID => "setpgid",
        GETPPID => "getppid",
        GETPGRP => "getpgrp",
        SIGACTION => "sigaction",
        SIGSUSPEND => "sigsuspend",
        GETGROUPS => "getgroups",
        SETGROUPS => "setgroups",
        REBOOT => "reboot",
        MMAP => "mmap",
        MUNMAP => "munmap",
        SOCKETCALL => "socketcall",
        CLONE => "clone",
        MPROTECT => "mprotect",
        SIGPROCMASK => "sigprocmask",
        GETPGID => "getpgid",
        NANOSLEEP => "nanosleep",
        SIGRETURN => "sigreturn",
        SHUTDOWN => "shutdown",
        TEST => "test",
        STACK_OVERFLOW => "stack_overflow",
        EXIT_QEMU => "exit_qemu",
        TCGETATTR => "tcgetattr",
        TCSETATTR => "tcsetattr",
        TCSETPGRP => "tcsetpgrp",
        TCGETPGRP => "tcgetpgrp",
        SETEGID => "setegid",
        SETEUID => "seteuid",
        ISATTY => "isatty",
        _ => "unknown syscall",
    };
    unpreemptible_context!({
        eprintln!("{} result: {:#?}", sysname, result);
    })
}
