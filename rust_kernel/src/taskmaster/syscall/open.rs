//! sys_open()

use super::SysResult;

use super::ipc::IpcResult;
use super::safe_ffi::{c_char, CString};
use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;

use core::convert::TryInto;

/// Open a new file descriptor
// TODO: Manage with the third argument
pub fn sys_open(filename: *const c_char, _flags: u32 /* mode */) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let file = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            // TODO: It will be usefull if a function returns a &str instead a CString
            let c_string: CString = (&v, filename).try_into()?;

            unsafe {
                core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    filename as *const u8,
                    c_string.len(),
                ))
            }
        };

        let fd_interface = &mut scheduler
            .current_thread_group_running_mut()
            .file_descriptor_interface;

        match fd_interface.open(file /* flags, mode etc... */)? {
            IpcResult::Wait(fd, file_op_uid) => {
                scheduler
                    .current_thread_mut()
                    .set_waiting(WaitingState::Open(file_op_uid));
                let _ret = auto_preempt()?;
                return Ok(fd);
            }
            IpcResult::Done(fd) => return Ok(fd),
        }
    })
}
