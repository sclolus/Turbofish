//! sys_open()
use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;
use super::IpcResult;
use super::SysResult;
use libc_binding::{c_char, mode_t, Errno, FileType, OpenFlags};

/// Open a new file descriptor
pub fn sys_open(filename: *const c_char, flags: u32, mode: mode_t) -> SysResult<u32> {
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

        let flags = OpenFlags::from_bits(flags).ok_or(Errno::EINVAL)?;
        let mode = FileType::from_bits(mode as u16).ok_or(Errno::EINVAL)?;
        match fd_interface.open(cwd, creds, file, flags, mode)? {
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
