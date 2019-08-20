//! sys_read()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, unpreemptible};
use super::thread::WaitingState;

use libc_binding::Errno;

use crate::terminal::{ReadResult, TERMINAL};

/// Read something from a file descriptor
pub fn sys_read(fd: i32, buf: *mut u8, count: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let controlling_terminal = scheduler.current_thread_group().controlling_terminal;
        let output = {
            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.make_checked_mut_slice(buf, count)?
        };

        if fd == 0 {
            // TODO: change that, read on tty 1 for the moment
            let read_result = unsafe {
                TERMINAL
                    .as_mut()
                    .unwrap()
                    .read(output, controlling_terminal)
            };

            // the read was non blocking
            if let ReadResult::NonBlocking(read_count) = read_result {
                return Ok(read_count as u32);
            }
            // else the read was blocking

            scheduler
                .current_thread_mut()
                .set_waiting(WaitingState::Read);
            let _ret = auto_preempt()?;

            unpreemptible();

            let read_result = unsafe {
                TERMINAL
                    .as_mut()
                    .unwrap()
                    .read(output, controlling_terminal)
            };
            match read_result {
                ReadResult::NonBlocking(read_count) => {
                    return Ok(read_count as u32);
                }
                ReadResult::Blocking => {
                    panic!("read has been wake up but there is nothing to read")
                }
            }
        } else {
            Err(Errno::EPERM)
        }
    })
}
