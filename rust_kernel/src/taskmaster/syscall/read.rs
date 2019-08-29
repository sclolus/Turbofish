//! sys_read()

use super::SysResult;

use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;
use super::IpcResult;

/// Read something from a file descriptor
pub fn sys_read(fd: i32, mut buf: *mut u8, mut count: usize) -> SysResult<u32> {
    let mut readen_bytes = 0;
    loop {
        unpreemptible_context!({
            let mut scheduler = SCHEDULER.lock();

            let output = {
                let v = scheduler
                    .current_thread_mut()
                    .unwrap_process_mut()
                    .get_virtual_allocator();

                // Check if pointer exists in user virtual address space
                v.make_checked_mut_slice(buf, count)?
            };

            let fd_interface = &mut scheduler
                .current_thread_group_running_mut()
                .file_descriptor_interface;

            match fd_interface.read(fd as _, output)? {
                IpcResult::Wait(res, file_op_uid) => {
                    readen_bytes += res;
                    buf = unsafe { buf.add(res as _) };
                    count -= res as usize;
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Read(file_op_uid));
                    let _ret = auto_preempt()?;
                }
                IpcResult::Done(res) => return Ok(readen_bytes + res),
            }
        })
    }
}
