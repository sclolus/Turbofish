//! sys_read()

use super::SysResult;

use super::ipc::{IpcResult, IpcStatus};
use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, preemptible};
use super::thread::WaitingState;

/// Read something from a file descriptor
pub fn sys_read(fd: i32, mut buf: *mut u8, mut count: usize) -> SysResult<u32> {
    let mut readen_bytes = 0;
    loop {
        unpreemptible_context!({
            let mut scheduler = SCHEDULER.lock();

            // First, check the current memory area and generate associated slice
            let output = {
                let v = scheduler
                    .current_thread_mut()
                    .unwrap_process_mut()
                    .get_virtual_allocator();

                // Check if pointer exists in user virtual address space
                v.make_checked_mut_slice(buf, count)?
            };

            let task = scheduler.current_thread_mut();

            let result = task.fd_interface.read(fd as _, output)?;

            match result {
                IpcResult {
                    res,
                    status: IpcStatus::Wait,
                } => {
                    // The read was blocking
                    readen_bytes += res;
                    // XXX: This part is highly unsafe
                    buf = unsafe { buf.add(res as _) };
                    count -= res as usize;
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Read);
                    let _ret = auto_preempt()?;
                }
                IpcResult {
                    res,
                    status: IpcStatus::Continue,
                } => {
                    preemptible();
                    readen_bytes += res;
                    return Ok(readen_bytes);
                }
            }
        })
    }
}
