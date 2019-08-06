//! sys_read()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, unpreemptible};
use super::task::WaitingState;

use errno::Errno;

use crate::terminal::TERMINAL;

fn read_from_terminal(buf: &mut [u8]) -> u32 {
    unsafe { TERMINAL.as_mut().unwrap().read(buf, 1) as u32 }
}

/// Read something from a file descriptor
pub fn sys_read(fd: i32, buf: *mut u8, count: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        {
            let v = scheduler
                .current_task_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.check_user_ptr_with_len::<u8>(buf, count)?;
        }
        let output = unsafe { core::slice::from_raw_parts_mut(buf, count) };

        if fd == 0 {
            // Auto-preempt calling
            // unsafe {
            //     KEY_SYMB_OPT = None;
            // Register callback
            // KEYBOARD_DRIVER
            //     .as_mut()
            //     .unwrap()
            //     .bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
            // }

            let read_count = read_from_terminal(output);
            if read_count != 0 {
                return Ok(read_count);
            }

            scheduler.current_task_mut().set_waiting(WaitingState::Read);
            let ret = auto_preempt();

            unpreemptible();

            if ret < 0 {
                return Err(Errno::Eintr);
            } else {
                // TODO: May be more bigger. TODO: Check size
                // TODO: Must be sizeof of readen character
                // println!("{:#X?}", ret);
                let read_count = read_from_terminal(output);

                if read_count == 0 {
                    panic!("read has been wake up but there is nothing to read");
                }
                return Ok(read_count);
            }
        } else {
            Err(Errno::Eperm)
        }
    })
}
