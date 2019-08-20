// <<<<<<< HEAD
// use super::scheduler::SCHEDULER;
// use super::SysResult;
// use libc_binding::Errno;

// /// Write something into the screen
// pub fn sys_write(fd: i32, buf: *const u8, count: usize) -> SysResult<u32> {
//     if fd != 1 && fd != 2 {
//         Err(Errno::EBADF)
//     } else {
//         unsafe {
//             unpreemptible_context!({
//                 let controlling_terminal =
//                     SCHEDULER.lock().current_thread_group().controlling_terminal;
//                 if fd == 2 {
//                     eprint!(
//                         "{}",
//                         core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count))
//                     );
//                 } else {
//                     print_tty!(
//                         controlling_terminal,
//                         "{}",
//                         core::str::from_utf8_unchecked(core::slice::from_raw_parts(buf, count))
//                     );
//                 }
//             })
//         }
//         Ok(count as u32)
// =======
//! sys_write()

use super::SysResult;

use super::ipc::IpcResult;
use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
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

            let fd_interface = &mut scheduler
                .current_thread_group_running_mut()
                .file_descriptor_interface;

            match fd_interface.write(fd as _, output)? {
                IpcResult::Wait(res) => {
                    written_bytes += res;
                    buf = unsafe { buf.add(res as _) };
                    count -= res as usize;
                    scheduler
                        .current_thread_mut()
                        .set_waiting(WaitingState::Write);
                    let _ret = auto_preempt()?;
                }
                IpcResult::Done(res) => return Ok(written_bytes + res),
            }
        })
    }
}
