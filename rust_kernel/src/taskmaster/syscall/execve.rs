//! sys_execve implementation

use super::SysResult;

use super::process::{CpuState, Process, TaskOrigin, UserProcess};
use super::safe_ffi::{c_char, CString, CStringArray};
use super::scheduler::SCHEDULER;
use super::task::ProcessState;

use alloc::format;
use alloc::vec::Vec;
use core::convert::TryInto;
use errno::Errno;
use ext2::syscall::OpenFlags;
use fallible_collections::try_vec;

use crate::drivers::storage::ext2::EXT2;

/// Return a file content using raw ext2 methods
fn get_file_content(pathname: &str) -> SysResult<Vec<u8>> {
    let ext2 = unsafe {
        EXT2.as_mut()
            .ok_or("ext2 not init")
            .map_err(|_| Errno::Enodev)?
    };

    let mut file = ext2.open(&pathname, OpenFlags::O_RDONLY, 0)?;

    let inode = ext2.get_inode(file.inode_nbr)?;

    let mut v: Vec<u8> = try_vec![0; inode.0.low_size as usize]?;

    let len = ext2.read(&mut file, v.as_mut())?;

    if len != inode.0.low_size as u64 {
        Err(Errno::Eio)
    } else {
        Ok(v)
    }
}

/// Execute a program
pub fn sys_execve(
    filename: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> SysResult<u32> {
    let argc = unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = scheduler
            .current_task_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();

        let filename: CString = (&v, filename).try_into()?;
        let argv_content: CStringArray = (&v, argv).try_into()?;
        let envp_content: CStringArray = (&v, envp).try_into()?;
        drop(v);

        // TODO: Use PWD later. (note that format! macro is not in a faillible memory context)
        let pathname = format!("/bin/{}", filename);

        let content = get_file_content(&pathname)?;

        let mut new_process = unsafe { UserProcess::new(TaskOrigin::Elf(content.as_ref()))? };

        let old_process = scheduler.current_task_mut().unwrap_process_mut();
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
        scheduler.current_task_mut().process_state = ProcessState::Running(new_process);

        // Reset the signal interface
        scheduler
            .current_task_mut()
            .signal
            .reset_for_new_process_image();

        let p = scheduler.current_task_mut().unwrap_process_mut();
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
