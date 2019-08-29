//! sys_write()

use super::SysResult;

use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;
use super::IpcResult;

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

            let fd_interface = &mut scheduler
                .current_thread_group_running_mut()
                .file_descriptor_interface;

            match fd_interface.write(fd as _, output)? {
                IpcResult::Wait(res, file_op_uid) => {
                    written_bytes += res;
                    buf = unsafe { buf.add(res as _) };
                    count -= res as usize;
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Write(file_op_uid));
                    let _ret = auto_preempt()?;
                }
                IpcResult::Done(res) => return Ok(written_bytes + res),
            }
        })
    }
}
