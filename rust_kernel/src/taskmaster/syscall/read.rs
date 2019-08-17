//! sys_read()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, unpreemptible};
use super::thread::WaitingState;

use errno::Errno;

use crate::terminal::{ReadResult, TERMINAL};

/// Read something from a file descriptor
pub fn sys_read(fd: i32, buf: *mut u8, count: usize) -> SysResult<u32> {
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

        if fd == 0 {
            // TODO: change that, read on tty 1 for the moment
            let read_result = unsafe { TERMINAL.as_mut().unwrap().read(output, 1) };

            // the read was non blocking
            if let ReadResult::NonBlocking(read_count) = read_result {
                return Ok(read_count as u32);
            }
            // else the read was blocking

            scheduler
                .current_thread_mut()
                .set_waiting(WaitingState::Read);
            let ret = auto_preempt();

            unpreemptible();

            if ret < 0 {
                return Err(Errno::Eintr);
            } else {
                // TODO: change that, read on tty 1 for the moment
                let read_result = unsafe { TERMINAL.as_mut().unwrap().read(output, 1) };
                match read_result {
                    ReadResult::NonBlocking(read_count) => {
                        return Ok(read_count as u32);
                    }
                    ReadResult::Blocking => {
                        panic!("read has been wake up but there is nothing to read")
                    }
                }
            }
        } else {
            Err(Errno::Eperm)
        }
    })
}
