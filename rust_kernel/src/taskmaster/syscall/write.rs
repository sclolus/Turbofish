//! sys_write()

use super::SysResult;

use super::ipc::IpcResult;
use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, preemptible};
use super::thread::WaitingState;

/// Write something into a file descriptor
pub fn sys_write(fd: i32, mut buf: *const u8, mut count: usize) -> SysResult<u32> {
    let mut written_bytes = 0;
    loop {
        unpreemptible_context!({
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

            match task.fd_interface.write(fd as _, output)? {
                IpcResult::Wait(res) => {
                    written_bytes += res;
                    buf = unsafe { buf.add(res as _) };
                    count -= res as usize;
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Write);
                    let _ret = auto_preempt()?;
                }
                IpcResult::Continue(res) => {
                    preemptible();
                    return Ok(written_bytes + res);
                }
            }
        })
    }
}
