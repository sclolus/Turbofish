//! all kernel syscall start by sys_ and userspace syscall (which will be in libc anyway) start by user_

use super::fd_interface::Fd;
use super::kmodules;
use super::process;
use super::process::CpuState;
use super::safe_ffi;
use super::scheduler;
use super::scheduler::{Pid, PreemptionGuard, SCHEDULER};
use super::signal_interface;
use super::signal_interface::{sigset_t, StructSigaction};
use super::thread;
use super::thread_group;
use super::vfs;
use super::IpcResult;
use super::{IntoRawResult, SysResult};
use libc_binding::{
    ACCESS, CHDIR, CHMOD, CHOWN, CLONE, CLOSE, DUP, DUP2, EXECVE, EXIT, EXIT_QEMU, FCHMOD, FCHOWN,
    FCNTL, FORK, FSTAT, FSTATFS, GETCWD, GETEGID, GETEUID, GETGID, GETGROUPS, GETPGID, GETPGRP,
    GETPID, GETPPID, GETTIMEOFDAY, GETUID, INSMOD, ISATTY, IS_STR_VALID, KILL, LINK, LSEEK, LSTAT,
    MKDIR, MKNOD, MMAP, MOUNT, MPROTECT, MUNMAP, NANOSLEEP, OPEN, OPENDIR, PAUSE, PIPE, READ,
    READLINK, REBOOT, RENAME, RMDIR, RMMOD, SETEGID, SETEUID, SETGID, SETGROUPS, SETPGID, SETUID,
    SHUTDOWN, SIGACTION, SIGNAL, SIGPROCMASK, SIGRETURN, SIGSUSPEND, SOCKETCALL, STACK_OVERFLOW,
    STAT, STATFS, SYMLINK, TCGETATTR, TCGETPGRP, TCSETATTR, TCSETPGRP, TEST, TIMES, UMASK, UMOUNT,
    UNLINK, UTIME, WAITPID, WRITE,
};

use core::ffi::c_void;
use i386::BaseRegisters;
use interrupts::idt::{GateType, IdtGateEntry, InterruptTable};
use libc_binding::Errno;
use libc_binding::{
    c_char, dev_t, gid_t, mode_t, off_t, termios, timeval, timezone, tms, uid_t, utimbuf, DIR,
};

mod mmap;
use mmap::{sys_mmap, MmapArgStruct};

mod nanosleep;
use nanosleep::{sys_nanosleep, TimeSpec};

mod gettimeofday;
use gettimeofday::sys_gettimeofday;

mod waitpid;
use waitpid::sys_waitpid;
pub use waitpid::WaitOption;

mod unlink;
use unlink::sys_unlink;

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

mod getpid;
use getpid::sys_getpid;

mod getppid;
use getppid::sys_getppid;

mod exit;
use exit::sys_exit;

mod setgroups;
use setgroups::sys_setgroups;

mod getgroups;
use getgroups::sys_getgroups;

mod setegid;
use setegid::sys_setegid;

mod seteuid;
use seteuid::sys_seteuid;

mod sigsuspend;
use sigsuspend::sys_sigsuspend;

mod signal;
use signal::sys_signal;

mod sigprocmask;
use sigprocmask::sys_sigprocmask;

mod sigaction;
use sigaction::sys_sigaction;

mod sigreturn;
use sigreturn::sys_sigreturn;

mod pause;
use pause::sys_pause;

mod kill;
pub use kill::sys_kill;

mod mprotect;
use mprotect::{sys_mprotect, MmapProt};

mod munmap;
use munmap::sys_munmap;

mod umask;
use umask::sys_umask;

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

mod getuid;
use getuid::sys_getuid;

mod setgid;
use setgid::sys_setgid;

mod setuid;
use setuid::sys_setuid;

mod getgid;
use getgid::sys_getgid;

mod geteuid;
use geteuid::sys_geteuid;

mod getegid;
use getegid::sys_getegid;

mod lseek;
use lseek::sys_lseek;

mod fcntl;
use fcntl::sys_fcntl;

mod opendir;
use opendir::sys_opendir;

mod stat;
use stat::{statfn, sys_stat};

mod lstat;
use lstat::sys_lstat;

mod fstat;
use fstat::sys_fstat;

mod statfs;
use statfs::sys_statfs;

mod fstatfs;
use fstatfs::sys_fstatfs;

mod chdir;
use chdir::sys_chdir;

mod getcwd;
use getcwd::sys_getcwd;

mod is_str_valid;
use is_str_valid::sys_is_str_valid;

mod access;
use access::sys_access;

mod chmod;
use chmod::sys_chmod;

mod chown;
use chown::sys_chown;

mod fchown;
use fchown::sys_fchown;

mod fchmod;
use fchmod::sys_fchmod;

mod link;
use link::sys_link;
mod mkdir;
use mkdir::sys_mkdir;
mod rmdir;
use rmdir::sys_rmdir;
mod rename;
use rename::sys_rename;
mod symlink;
use symlink::sys_symlink;
mod mknod;
use mknod::sys_mknod;
mod utime;
use utime::sys_utime;
mod readlink;
use readlink::sys_readlink;

/*
 * These below declarations are IPC related
 */
mod dup;
use dup::sys_dup;
mod dup2;
use dup2::sys_dup2;
mod pipe;
use pipe::sys_pipe;
pub mod socket;
use socket::{sys_socketcall, SocketArgsPtr};
mod read;
use read::sys_read;
mod write;
use write::sys_write;
mod open;
use open::sys_open;
mod close;
use close::sys_close;
mod isatty;
use isatty::sys_isatty;
mod umount;
use umount::sys_umount;
mod mount;
use mount::sys_mount;

/*
 * Module kernel management
 */
mod insmod;
use insmod::sys_insmod;
mod rmmod;
use rmmod::sys_rmmod;

mod times;
use times::sys_times;

mod trace_syscall;

extern "C" {
    fn _isr_syscall();

    fn _get_pit_time() -> u32;
    fn _get_process_end_time() -> u32;
}

/// Global syscall interrupt handler called from assembly code
/// See https://www.informatik.htw-dresden.de/~beck/ASM/syscall_list.html
/// This function returns a pointer on a process stack to follow
#[no_mangle]
pub unsafe extern "C" fn syscall_interrupt_handler(cpu_state: *mut CpuState) -> u32 {
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

    if eax != READ && eax != WRITE {
        // trace_syscall::trace_syscall(cpu_state);
    }
    let result = match eax {
        EXIT => sys_exit(ebx as i32),       // This syscall doesn't return !
        FORK => sys_fork(cpu_state as u32), // CpuState represents kernel_esp
        READ => sys_read(ebx as i32, ecx as *mut u8, edx as usize),
        WRITE => sys_write(ebx as i32, ecx as *const u8, edx as usize),
        OPEN => sys_open(ebx as *const c_char, ecx as u32, edx as mode_t),
        CLOSE => sys_close(ebx as i32),
        WAITPID => sys_waitpid(ebx as i32, ecx as *mut i32, edx as u32),
        LINK => sys_link(ebx as *const c_char, ecx as *const c_char),
        UNLINK => sys_unlink(ebx as *const c_char),
        STAT => sys_stat(ebx as *const c_char, ecx as *mut libc_binding::stat),
        EXECVE => sys_execve(
            ebx as *const c_char,
            ecx as *const *const c_char,
            edx as *const *const c_char,
        ),
        CHDIR => sys_chdir(ebx as *const c_char),
        CHMOD => sys_chmod(ebx as *const c_char, ecx as mode_t),
        FCHMOD => sys_fchmod(ebx as Fd, ecx as mode_t),
        MKNOD => sys_mknod(ebx as *const c_char, ecx as mode_t, edx as dev_t),
        LSEEK => sys_lseek(
            ebx as *mut off_t,
            ecx as Fd,
            edx as off_t + ((esi as off_t) << 32),
            edi as u32,
        ),
        GETPID => sys_getpid(),
        MOUNT => sys_mount(
            ebx as *const c_char,
            ecx as *const c_char,
            edx as *const c_char,
            edi as u32,
            ebp as *const c_void,
        ),
        SETUID => sys_setuid(ebx as uid_t),
        GETUID => sys_getuid(),
        PAUSE => sys_pause(),
        FSTAT => sys_fstat(ebx as Fd, ecx as *mut libc_binding::stat),
        ACCESS => sys_access(ebx as *const c_char, ecx as u32),
        UTIME => sys_utime(ebx as *const c_char, ecx as *const utimbuf),
        KILL => sys_kill(ebx as i32, ecx as u32),
        RENAME => sys_rename(ebx as *const c_char, ecx as *const c_char),
        MKDIR => sys_mkdir(ebx as *const c_char, ecx as mode_t),
        RMDIR => sys_rmdir(ebx as *const c_char),
        PIPE => sys_pipe(core::slice::from_raw_parts_mut(ebx as *mut i32, 2)),
        TIMES => sys_times(ebx as *mut tms),
        DUP => sys_dup(ebx as u32),
        SETGID => sys_setgid(ebx as gid_t),
        GETGID => sys_getgid(),
        GETEUID => sys_geteuid(),
        FCNTL => sys_fcntl(ebx as Fd, ecx as u32, edx as Fd),
        GETEGID => sys_getegid(),
        UMOUNT => sys_umount(ebx as *const c_char),
        SIGNAL => sys_signal(ebx as u32, ecx as usize),
        SETPGID => sys_setpgid(ebx as Pid, ecx as Pid),
        DUP2 => sys_dup2(ebx as u32, ecx as u32),
        GETPPID => sys_getppid(),
        GETPGRP => sys_getpgrp(),
        SIGACTION => sys_sigaction(
            ebx as u32,
            ecx as *const StructSigaction,
            edx as *mut StructSigaction,
        ),
        SIGSUSPEND => sys_sigsuspend(ebx as *const sigset_t),
        GETGROUPS => sys_getgroups(ebx as i32, ecx as *mut gid_t),
        SETGROUPS => sys_setgroups(ebx as i32, ecx as *const gid_t),
        SYMLINK => sys_symlink(ebx as *const c_char, ecx as *const c_char),
        LSTAT => sys_lstat(ebx as *const c_char, ecx as *mut libc_binding::stat),
        READLINK => sys_readlink(ebx as *const c_char, ecx as *mut c_char, edx as u32),
        REBOOT => sys_reboot(),
        MMAP => sys_mmap(ebx as *const MmapArgStruct),
        MUNMAP => sys_munmap(ebx as *mut u8, ecx as usize),
        UMASK => sys_umask(ebx as mode_t),
        SOCKETCALL => sys_socketcall(ebx as u32, ecx as SocketArgsPtr),
        CLONE => sys_clone(cpu_state as u32, ebx as *const c_void, ecx as u32),
        MPROTECT => sys_mprotect(
            ebx as *mut u8,
            ecx as usize,
            MmapProt::from_bits_truncate(edx),
        ),
        SIGPROCMASK => sys_sigprocmask(ebx as u32, ecx as *const sigset_t, edx as *mut sigset_t),
        GETPGID => sys_getpgid(ebx as Pid),
        STATFS => sys_statfs(ebx as *const c_char, ecx as *mut libc_binding::statfs),
        FSTATFS => sys_fstatfs(ebx as Fd, ecx as *mut libc_binding::statfs),
        NANOSLEEP => sys_nanosleep(ebx as *const TimeSpec, ecx as *mut TimeSpec),
        CHOWN => sys_chown(ebx as *const c_char, ecx as uid_t, edx as gid_t),
        FCHOWN => sys_fchown(ebx as Fd, ecx as uid_t, edx as gid_t),
        GETCWD => sys_getcwd(ebx as *mut c_char, ecx as usize),
        GETTIMEOFDAY => sys_gettimeofday(ebx as *mut timeval, ecx as *mut timezone),
        SIGRETURN => sys_sigreturn(cpu_state),
        SHUTDOWN => sys_shutdown(),
        TEST => sys_test(),
        STACK_OVERFLOW => sys_stack_overflow(0, 0, 0, 0, 0, 0),
        EXIT_QEMU => crate::tests::helpers::exit_qemu(ebx as u32),
        TCGETATTR => sys_tcgetattr(ebx as Fd, ecx as *mut termios),
        TCSETATTR => sys_tcsetattr(ebx as Fd, ecx as u32, edx as *const termios),
        TCSETPGRP => sys_tcsetpgrp(ebx as Fd, ecx as Pid),
        TCGETPGRP => sys_tcgetpgrp(ebx as Fd),
        SETEGID => sys_setegid(ebx as gid_t),
        SETEUID => sys_seteuid(ebx as uid_t),
        ISATTY => sys_isatty(ebx as u32),
        OPENDIR => sys_opendir(ebx as *const c_char, ecx as *mut DIR),
        IS_STR_VALID => sys_is_str_valid(ebx as *const c_char),

        // Kernel module management
        INSMOD => sys_insmod(ebx as *const c_char),
        RMMOD => sys_rmmod(ebx as *const c_char),

        // set thread area: WTF
        0xf3 => Err(Errno::EPERM),
        sysnum => panic!("wrong syscall {}", sysnum),
    };

    if eax != READ && eax != WRITE {
        // trace_syscall::trace_syscall_result(cpu_state, result);
    }

    let is_in_blocked_syscall = result == Err(Errno::EINTR);
    // Note: do not erase eax if we've just been interrupted from a blocked syscall as we must keep
    // the syscall number contained in eax, in case of SA_RESTART behavior
    if is_in_blocked_syscall == false {
        // Return value will be on EAX. Errno always represents the low 7 bits
        (*cpu_state).registers.eax = result.into_raw_result();
    }
    exit_from_syscall(cpu_state, is_in_blocked_syscall)
}

fn exit_from_syscall(cpu_state: *mut CpuState, is_in_blocked_syscall: bool) -> u32 {
    let mut preemption_guard = PreemptionGuard::new();
    let mut scheduler = SCHEDULER.lock();
    // An exit() routine may be engaged by the exit() syscall - An exit() routine is already on execution
    if let Some(_) = scheduler.on_exit_routine {
        scheduler.set_dustman_mode()
    } else {
        // If ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame. UNLOCK interruptible().
        // If ring0 process -> Can't happened normally
        if let Some(_) =
            scheduler.current_thread_deliver_pending_signals(cpu_state, is_in_blocked_syscall)
        {
            // An exit() routine may be engaged after handling a deadly signal - An exit() routine is already on execution - block interrupts
            preemption_guard.set_already_unpreemptible();
            scheduler.set_dustman_mode()
        } else {
            cpu_state as u32
        }
    }
}

extern "C" {
    fn _schedule_next();
}

/// Initialize all the syscall system by creation of a new IDT entry at 0x80
pub fn init() {
    let mut interrupt_table = unsafe { InterruptTable::current_interrupt_table() };

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(3)
        .set_selector(1 << 3)
        .set_gate_type(GateType::TrapGate32);
    gate_entry.set_handler(_isr_syscall as *const c_void as u32);
    interrupt_table[0x80] = gate_entry;
}
