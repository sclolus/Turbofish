use super::scheduler::SCHEDULER;
use super::SysResult;
use bitflags::bitflags;
use core::ffi::c_void;

bitflags! {
    pub struct CloneFlags: u32 {
        // const CSIGNAL: u32 = 0x000000ff; /* signal mask to be sent at exit */
        const VM = 0x00000100; /* set if VM shared between processes */
        const FS = 0x00000200; /* set if fs info shared between processes */
        const FILES = 0x00000400; /* set if open files shared between processes */
        const SIGHAND = 0x00000800; /* set if signal handlers and blocked signals shared */
        const PTRACE = 0x00002000; /* set if we want to let tracing continue on the child too */
        const VFORK = 0x00004000; /* set if the parent wants the child to wake it up on mm_release */
        const PARENT = 0x00008000; /* set if we want to have the same parent as the cloner */
        const THREAD = 0x00010000; /* Same thread group? */
        const NEWNS = 0x00020000; /* New mount namespace group */
        const SYSVSEM = 0x00040000; /* share system V SEM_UNDO semantics */
        const SETTLS = 0x00080000; /* create a new TLS for the child */
        const PARENT_SETTID = 0x00100000; /* set the TID in the parent */
        const CHILD_CLEARTID = 0x00200000; /* clear the TID in the child */
        const DETACHED = 0x00400000; /* Unused; ignored */
        const UNTRACED = 0x00800000; /* set if the tracing process can't force CLONE_PTRACE on this clone */
        const CHILD_SETTID = 0x01000000; /* set the TID in the child */
        const NEWCGROUP = 0x02000000; /* New cgroup namespace */
        const NEWUTS = 0x04000000; /* New utsname namespace */
        const NEWIPC = 0x08000000; /* New ipc namespace */
        const NEWUSER = 0x10000000; /* New user namespace */
        const NEWPID = 0x20000000; /* New pid namespace */
        const NEWNET = 0x40000000; /* New network namespace */
        const IO = 0x80000000; /* Clone io context */
    }
}
// a call to fork: clone(child_stack=NULL,
// flags=CLONE_CHILD_CLEARTID|CLONE_CHILD_SETTID|SIGCHLD,
// child_tidptr=0x7f2424406790) = 21725 a call to create_thread:
// clone(child_stack=0x7ff03ba94e70,
// flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID,
// parent_tidptr=0x7ff03ba959d0, tls=0x7ff03ba95700,
// child_tidptr=0x7ff03ba959d0) = 21807
pub fn sys_clone(kernel_esp: u32, child_stack: *const c_void, clone_flags: u32) -> SysResult<u32> {
    let flags = CloneFlags::from_bits_truncate(clone_flags);

    unpreemptible_context!({
        SCHEDULER
            .lock()
            .current_task_clone(kernel_esp, child_stack, flags)
    })
}
