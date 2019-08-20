//! This file contains the description of the pipe syscall

use super::scheduler::SCHEDULER;
use super::SysResult;

/// Create pipe
pub fn sys_pipe(fd: &mut [i32]) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr_with_len::<i32>(fd.as_ptr(), core::mem::size_of::<[i32; 2]>())?;
        }
        let fd_interface = &mut scheduler
            .current_thread_group_running_mut()
            .file_descriptor_interface;

        let ret = fd_interface.new_pipe()?;
        fd[0] = ret.0 as _;
        fd[1] = ret.1 as _;
    });
    Ok(0)
}
