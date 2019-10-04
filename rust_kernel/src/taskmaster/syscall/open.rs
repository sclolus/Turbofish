//! sys_open()
use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;
use super::vfs::VFS;
use super::IpcResult;
use super::SysResult;
use libc_binding::{c_char, mode_t, Errno, FileType, OpenFlags};

/// Open a new file descriptor
pub fn sys_open(filename: *const c_char, flags: u32, mut mode: mode_t) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let file = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_str(filename)?
        };

        let flags = OpenFlags::from_bits(flags).ok_or(Errno::EINVAL)?;

        let fd = {
            let creds;
            let cwd;
            let fd_interface;
            let umask;
            {
                let tg = scheduler.current_thread_group_mut();

                creds = &tg.credentials;
                cwd = &tg.cwd;
                fd_interface = &mut tg
                    .thread_group_state
                    .unwrap_running_mut()
                    .file_descriptor_interface;
                umask = tg.umask;
            }

            // Mask out the bits of mode which are set in umask.
            mode &= !umask;

            let mode = FileType::from_bits(mode as u16).ok_or(Errno::EINVAL)?;

            match fd_interface.open(cwd, creds, file, flags, mode)? {
                IpcResult::Wait(fd, file_op_uid) => {
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Open(file_op_uid));
                    let _ret = auto_preempt()?;
                    fd
                }
                IpcResult::Done(fd) => fd,
            }
        };

        let tg = scheduler.current_thread_group_mut();
        let fd_interface = &tg
            .thread_group_state
            .unwrap_running_mut()
            .file_descriptor_interface;

        let mut file_operation = fd_interface
            .get_file_operation(fd)
            .expect("FileOperation should be there for fd.");

        if file_operation.isatty() == Ok(1) && !flags.contains(OpenFlags::O_NOCTTY) {
            let inode_id = file_operation
                .get_inode_id()
                .expect("Should be possible to get the inode id of a tty");
            let tty_minor = VFS
                .lock()
                .get_inode(inode_id)
                .expect("Inode should be there")
                .minor;

            tg.controlling_terminal = Some(tty_minor);
        }
        Ok(fd)
    })
}
