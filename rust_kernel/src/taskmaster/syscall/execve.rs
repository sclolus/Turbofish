//! sys_execve implementation

use super::SysResult;

use super::process::{CpuState, Process, ProcessOrigin, UserProcess};
use super::safe_ffi::{c_char, CString, CStringArray};
use super::scheduler::SCHEDULER;
use super::thread::ProcessState;

use alloc::format;
use alloc::vec::Vec;
use core::convert::TryInto;
use ext2::syscall::OpenFlags;
use fallible_collections::try_vec;
use libc_binding::Errno;

use crate::drivers::storage::ext2::EXT2;

/// Return a file content using raw ext2 methods
fn get_file_content(pathname: &str) -> SysResult<Vec<u8>> {
    let ext2 = unsafe {
        EXT2.as_mut()
            .ok_or("ext2 not init")
            .map_err(|_| Errno::ENODEV)?
    };

    let mut file = ext2.open(&pathname, OpenFlags::O_RDONLY, 0)?;

    let inode = ext2.get_inode(file.inode_nbr)?;

    let mut v: Vec<u8> = try_vec![0; inode.0.low_size as usize]?;

    let len = ext2.read(&mut file, v.as_mut())?;

    if len != inode.0.low_size as u64 {
        Err(Errno::EIO)
    } else {
        Ok(v)
    }
}
/// File descriptors open in the calling process image shall remain
/// open in the new process image, except for those whose close-on-
/// exec flag FD_CLOEXEC is set. For those file descriptors that
/// remain open, all attributes of the open file description remain
/// unchanged. For any file descriptor that is closed for this reason,
/// file locks are removed as a result of the close as described in
/// close(). Locks that are not removed by closing of file descriptors
/// remain unchanged.
///
/// Directory streams open in the calling process image shall be
/// closed in the new process image.
///
/// Signals set to the default action (SIG_DFL) in the calling process
/// image shall be set to the default action in the new process
/// image. Except for SIGCHLD, signals set to be ignored (SIG_IGN) by
/// the calling process image shall be set to be ignored by the new
/// process image. Signals set to be caught by the calling process
/// image shall be set to the default action in the new process image
/// (see <signal.h>).
///
/// After a successful call to any of the exec functions, alternate
/// signal stacks are not preserved and the SA_ONSTACK flag shall be
/// cleared for all signals.
/// If the SIGCHLD signal is set to be ignored by the calling process
/// image, it is unspecified whether the SIGCHLD signal is set to be
/// ignored or to the default action in the new process image.
/// Execute a program
///
///The new process shall inherit at least the following attributes from the calling process image:
///
///    [XSI] [Option Start] Nice value (see nice()) [Option End]
///
///    [XSI] [Option Start] semadj values (see semop()) [Option End]
///
///    Process ID
///
///    Parent process ID
///
///    Process group ID
///
///    Session membership
///
///    Real user ID
///
///    Real group ID
///
///    Supplementary group IDs
///
///    Time left until an alarm clock signal (see alarm())
///
///    Current working directory
///
///    Root directory
///
///    File mode creation mask (see umask())
///
///    [XSI] [Option Start] File size limit (see getrlimit() and setrlimit()) [Option End]
///
///    Process signal mask (see pthread_sigmask())
///
///    Pending signal (see sigpending())
///
///    tms_utime, tms_stime, tms_cutime, and tms_cstime (see times())
///
///    [XSI] [Option Start] Resource limits [Option End]
///
///    Controlling terminal
///
///    [XSI] [Option Start] Interval timers [Option End]
///
///The initial thread of the new process shall inherit at least the following attributes from the calling thread:
///
///    Signal mask (see sigprocmask() and pthread_sigmask())
///
///    Pending signals (see sigpending())
///
// TODO:
/// A call to any exec function from a process with more than one
/// thread shall result in all threads being terminated and the new
/// executable image being loaded and executed. No destructor
/// functions or cleanup handlers shall be called.
pub fn sys_execve(
    filename: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> SysResult<u32> {
    let argc = unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();

        let filename: CString = (&v, filename).try_into()?;
        let argv_content: CStringArray = (&v, argv).try_into()?;
        let envp_content: CStringArray = (&v, envp).try_into()?;
        drop(v);

        // TODO: Use PWD later. (note that format! macro is not in a faillible memory context)
        let pathname = format!("{}", filename);

        if !(pathname.starts_with("/")) {
            unimplemented!();
        }
        let content = get_file_content(&pathname)?;

        let mut new_process = unsafe { UserProcess::new(ProcessOrigin::Elf(content.as_ref()))? };

        let old_process = scheduler.current_thread_mut().unwrap_process_mut();
        /*
         * We cannot move directly into the new process kernel stack, or just copy its content,
         * because rust made some optimizations with current process kernel stack.
         * So the trick is to exchange kernel stacks between old and new process.
         * We need also to save new CpuState before doing this operation, and finally switch the virtual context
         */
        unsafe {
            (old_process
                .kernel_stack
                .as_ptr()
                .add(old_process.kernel_stack.len() - core::mem::size_of::<CpuState>())
                as *mut u8)
                .copy_from(
                    new_process
                        .kernel_stack
                        .as_ptr()
                        .add(new_process.kernel_stack.len() - core::mem::size_of::<CpuState>()),
                    core::mem::size_of::<CpuState>(),
                );
        }
        core::mem::swap(&mut new_process.kernel_stack, &mut old_process.kernel_stack);

        unsafe {
            /*
             * Switch to the new virtual allocator context
             * IMPORTANT: Because of the TSS re-initialization. It is important to do that after swapping kernel stack
             */
            new_process.context_switch();
        }

        /*
         * Now, we can drop safety the old process
         */
        scheduler.current_thread_mut().process_state = ProcessState::Running(new_process);

        // Reset the signal interface
        scheduler
            .current_thread_mut()
            .signal
            .reset_for_new_process_image();

        let p = scheduler.current_thread_mut().unwrap_process_mut();
        let cpu_state = unsafe {
            p.kernel_stack
                .as_ptr()
                .add(p.kernel_stack.len() - core::mem::size_of::<CpuState>())
                as *mut CpuState
        };

        let align = 4;
        unsafe {
            // Set the argv argument: EBX
            (*cpu_state).esp -= argv_content.get_serialized_len(align).expect("WTF") as u32;
            (*cpu_state).registers.ebx = argv_content
                .serialize(align, (*cpu_state).esp as *mut c_char)
                .expect("WTF") as u32;
            // set the envp argument: ECX
            (*cpu_state).esp -= envp_content.get_serialized_len(align).expect("WTF") as u32;
            (*cpu_state).registers.ecx = envp_content
                .serialize(align, (*cpu_state).esp as *mut c_char)
                .expect("WTF") as u32;
        }
        // Set the argc argument: EAX
        argv_content.len() as u32
    });
    Ok(argc)
}
