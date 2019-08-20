//! This file contains the description of the write syscall

use super::SysResult;

use super::scheduler::SCHEDULER;

/// Write something into the screen
pub fn sys_write(fd: i32, buf: *const u8, count: usize) -> SysResult<u32> {
    let ret = unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let output = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.make_checked_slice(buf, count)?
        };

        let task = scheduler.current_thread_mut();

        task.fd_interface.write(fd as _, output)?.res
    });
    Ok(ret)
}
