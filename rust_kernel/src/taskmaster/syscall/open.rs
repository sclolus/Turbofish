//! sys_open()

use super::SysResult;

use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;
use super::IpcResult;
use libc_binding::c_char;

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

            v.make_checked_str(filename)?
        };

        let tg = scheduler.current_thread_group_mut();

        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        let fd_interface = &mut tg
            .thread_group_state
            .unwrap_running_mut()
            .file_descriptor_interface;

        match fd_interface.open(cwd, creds, file)? {
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
