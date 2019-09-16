use super::mmap::MmapArgStruct;
use super::nanosleep::TimeSpec;
use super::process::CpuState;
use super::signal_interface::{sigset_t, StructSigaction};
use super::Fd;
use super::MmapProt;
use super::SocketArgsPtr;
use super::SysResult;
use crate::memory::tools::address::Virt;
use crate::system::BaseRegisters;
use core::ffi::c_void;
use libc_binding::{
    c_char, dev_t, gid_t, mode_t, off_t, stat, termios, timeval, timezone, uid_t, utimbuf,
    OpenFlags, Pid, DIR,
};
use libc_binding::{
    ACCESS, CHDIR, CHMOD, CHOWN, CLONE, CLOSE, DUP, DUP2, EXECVE, EXIT, EXIT_QEMU, FCHMOD, FCHOWN,
    FCNTL, FORK, FSTAT, GETCWD, GETEGID, GETEUID, GETGID, GETGROUPS, GETPGID, GETPGRP, GETPID,
    GETPPID, GETTIMEOFDAY, GETUID, INSMOD, ISATTY, KILL, LINK, LSEEK, LSTAT, MKDIR, MKNOD, MMAP,
    MPROTECT, MUNMAP, NANOSLEEP, OPEN, OPENDIR, PAUSE, PIPE, READ, READLINK, REBOOT, RENAME, RMDIR,
    RMMOD, SETEGID, SETEUID, SETGID, SETGROUPS, SETPGID, SETUID, SHUTDOWN, SIGACTION, SIGNAL,
    SIGPROCMASK, SIGRETURN, SIGSUSPEND, SOCKETCALL, STACK_OVERFLOW, STAT, SYMLINK, TCGETATTR,
    TCGETPGRP, TCSETATTR, TCSETPGRP, TEST, UMASK, UNLINK, UTIME, WAITPID, WRITE,
};

#[allow(dead_code)]
pub fn trace_syscall(cpu_state: *mut CpuState) {
    let BaseRegisters {
        eax,
        ebx,
        ecx,
        edx,
        esi,
        edi,
        // ebp,
        ..
    } = unsafe { (*cpu_state).registers };
    unpreemptible_context!({
        match eax {
            EXIT => log::info!("exit({:#?})", ebx as i32),
            FORK => log::info!("fork()"),
            READ => log::info!(
                "read({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as *mut u8,
                edx as usize
            ),
            WRITE => log::info!(
                "write({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as *const u8,
                edx as usize
            ),
            // TODO: type parameter are not set and manage the third argument
            OPEN => log::info!(
                "open({:#?}, {:#?})",
                ebx as *const u8,
                OpenFlags::from_bits(ecx as u32)
            ),
            CLOSE => log::info!("close({:#?})", ebx as i32),
            WAITPID => log::info!(
                "waitpid({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as *mut i32,
                edx as i32
            ),
            UNLINK => log::info!("unlink({:#?})", ebx as *const u8),
            LINK => log::info!(
                "link({:#?}, {:#?})",
                ebx as *const c_char,
                ecx as *const c_char,
            ),
            EXECVE => log::info!(
                "execve({:#?}, {:#?}, {:#?})",
                ebx as *const c_char,
                ecx as *const *const c_char,
                edx as *const *const c_char,
            ),
            CHDIR => log::info!("chdir({:#?})", ebx as *const c_char),
            CHMOD => log::info!("chmod({:#?}, {:#?})", ebx as *const c_char, ecx as mode_t),
            FCHMOD => log::info!("fchmod({:#?}, {:#?})", ebx as Fd, ecx as mode_t),
            MKNOD => log::info!(
                "mknod({:#?}, {:#?}, {:#?})",
                ebx as *const c_char,
                ecx as mode_t,
                edx as dev_t,
            ),
            STAT => log::info!(
                "stat(filename: {:?}, buf: {:#X?})",
                ebx as *const c_char,
                ecx as *mut stat
            ),
            LSEEK => log::info!(
                "lseek(ptr: {:#?}, fd: {:#?}, offset: {:#?}, whence_value: {:#?})",
                ebx as *mut off_t,
                ecx as Fd,
                edx as off_t + ((esi as off_t) << 32),
                edi as u32,
            ),
            GETPID => log::info!("getpid()"),
            SETUID => log::info!("setuid({:#?})", ebx as uid_t),
            GETUID => log::info!("getuid()"),
            PAUSE => log::info!("pause()"),
            FSTAT => log::info!("fstat(fd: {:?}, buf: {:#X?})", ebx as Fd, ecx as *mut stat),
            ACCESS => log::info!("access({:#?}, {:#?})", ebx as *const c_char, ecx as i32),
            UTIME => log::info!(
                "utime({:#?}, {:#?})",
                ebx as *const libc_binding::c_char,
                ecx as *const utimbuf
            ),
            KILL => log::info!("kill({:#?}, {:#?})", ebx as i32, ecx as u32),
            RENAME => log::info!(
                "rename(
{:#?}, {:#?})",
                ebx as *const c_char,
                ecx as *const c_char,
            ),
            MKDIR => log::info!("mkdir({:#?}, {:#?})", ebx as *const c_char, ecx as mode_t),
            RMDIR => log::info!("rmdir({:#?})", ebx as *const c_char),
            PIPE => log::info!("pipe({:#?})", ebx as *const i32),
            DUP => log::info!("dup({:#?})", ebx as u32),
            SETGID => log::info!("setgid({:#?})", ebx as gid_t),
            GETGID => log::info!("getgid()"),
            GETEUID => log::info!("geteuid()"),
            FCNTL => log::info!(
                "fcntl({:#?}, {:#?}, {:#?})",
                ebx as Fd,
                ecx as u32,
                edx as Fd
            ),
            GETEGID => log::info!("getegid()"),
            SIGNAL => log::info!("signal({:#?}, {:#?})", ebx as u32, ecx as usize),
            SETPGID => log::info!("setpgid({:#?}, {:#?})", ebx as Pid, ecx as Pid),
            GETPPID => log::info!("getppid()"),
            DUP2 => log::info!("dup2({:#?}, {:#?})", ebx as u32, ecx as u32),
            GETPGRP => log::info!("getpgrp()"),
            SIGACTION => log::info!(
                "sigaction({:#?}, {:#?}, {:#?})",
                ebx as u32,
                ecx as *const StructSigaction,
                edx as *mut StructSigaction,
            ),
            SIGSUSPEND => log::info!("sigsuspend({:#?})", ebx as *const sigset_t),
            GETGROUPS => log::info!("getgroups({:#?}, {:#?})", ebx as i32, ecx as *mut gid_t),
            SETGROUPS => log::info!("setgroups({:#?}, {:#?})", ebx as i32, ecx as *const gid_t),
            SYMLINK => log::info!(
                "symlink({:#?}, {:#?})",
                ebx as *const c_char,
                ecx as *const c_char,
            ),
            LSTAT => log::info!("lstat(fd: {:?}, buf: {:#X?})", ebx as Fd, ecx as *mut stat),
            READLINK => log::info!(
                "readlink({:#?}, {:#?}, {:#?})",
                ebx as *const c_char,
                ecx as *mut c_char,
                edx as u32
            ),
            REBOOT => log::info!("reboot()"),
            MMAP => unsafe { log::info!("mmap({:#?})", *(ebx as *const MmapArgStruct)) },
            MUNMAP => log::info!("munmap({:#?}, {:#?})", Virt(ebx as usize), ecx as usize),
            UMASK => log::info!("umask({:#?})", ebx as mode_t),
            GETTIMEOFDAY => log::info!(
                "gettimeofday({:#?}, {:#?})",
                ebx as *mut timeval,
                ecx as *mut timezone
            ),
            SOCKETCALL => log::info!("socketcall({:#?}, {:#?})", ebx as u32, ecx as SocketArgsPtr),
            CLONE => log::info!(
                "clone({:#?}, {:#?}, {:#?})",
                cpu_state as u32,
                ebx as *const c_void,
                ecx as u32
            ),
            MPROTECT => log::info!(
                "mprotect({:#?}, {:#?}, {:#?})",
                Virt(ebx as usize),
                ecx as usize,
                MmapProt::from_bits_truncate(edx),
            ),
            SIGPROCMASK => log::info!(
                "sigprocmask({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as *const sigset_t,
                edx as *mut sigset_t
            ),
            GETPGID => log::info!("getpgid({:#?})", ebx as Pid),
            NANOSLEEP => log::info!(
                "nanosleep({:#?}, {:#?})",
                ebx as *const TimeSpec,
                ecx as *mut TimeSpec
            ),
            CHOWN => log::info!(
                "chown({:#?}, {:#?}, {:#?})",
                ebx as *const c_char,
                ecx as uid_t,
                edx as gid_t,
            ),

            FCHOWN => log::info!(
                "fchown({:#?}, {:#?}, {:#?})",
                ebx as Fd,
                ecx as uid_t,
                edx as gid_t,
            ),

            GETCWD => log::info!("getcwd({:#?}, {:#?})", ebx as *const c_char, ecx as usize),
            SIGRETURN => log::info!("sigreturn({:#?})", cpu_state),
            SHUTDOWN => log::info!("shutdown()"),
            TEST => log::info!("test()"),
            STACK_OVERFLOW => log::info!("stack_overflow()"),
            EXIT_QEMU => log::info!("exit_qemu({:#?})", ebx as u32),
            TCGETATTR => log::info!("tcgetattr({:#?}, {:#?})", ebx as i32, ecx as *mut termios),
            TCSETATTR => log::info!(
                "tcsetattr({:#?}, {:#?}, {:#?})",
                ebx as i32,
                ecx as u32,
                edx as *const termios
            ),
            TCSETPGRP => log::info!("tcsetpgrp({:#?}, {:#?})", ebx as i32, ecx as Pid),
            TCGETPGRP => log::info!("tcgetpgrp({:#?})", ebx as i32),
            SETEGID => log::info!("setegid({:#?})", ebx as gid_t),
            SETEUID => log::info!("seteuid({:#?})", ebx as uid_t),
            ISATTY => log::info!("isatty({:#?})", ebx as u32),
            OPENDIR => log::info!("opendir({:#?}, {:#?})", ebx as *const u8, ecx as *mut DIR),
            INSMOD => log::info!("insmod({:#?})", ebx as *const c_char),
            RMMOD => log::info!("rmmod({:#?})", ebx as *const c_char),
            unknown => log::info!("unknown syscall: {}", unknown),
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
        OPEN => "open",
        CLOSE => "close",
        WAITPID => "waitpid",
        LINK => "link",
        UNLINK => "unlink",
        EXECVE => "execve",
        CHDIR => "chdir",
        CHMOD => "chmod",
        FCHMOD => "fchmod",
        STAT => "stat",
        LSEEK => "lseek",
        GETPID => "getpid",
        SETUID => "setuid",
        GETUID => "getuid",
        PAUSE => "pause",
        FSTAT => "fstat",
        UTIME => "utime",
        ACCESS => "access",
        KILL => "kill",
        RENAME => "rename",
        MKDIR => "mkdir",
        RMDIR => "rmdir",
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
        LSTAT => "lstat",
        READLINK => "readlink",
        REBOOT => "reboot",
        MMAP => "mmap",
        MUNMAP => "munmap",
        UMASK => "umask",
        GETTIMEOFDAY => "gettimeofday",
        SOCKETCALL => "socketcall",
        CLONE => "clone",
        MPROTECT => "mprotect",
        SIGPROCMASK => "sigprocmask",
        GETPGID => "getpgid",
        CHOWN => "chown",
        FCHOWN => "fchown",
        NANOSLEEP => "nanosleep",
        GETCWD => "getcwd",
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
        OPENDIR => "opendir",
        INSMOD => "insmod",
        RMMOD => "rmmod",
        _ => "unknown syscall",
    };
    unpreemptible_context!({
        log::info!("{} result: {:#?}", sysname, result);
    })
}
