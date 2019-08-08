//! sys_read()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, unpreemptible};
use super::task::WaitingState;

use keyboard::keysymb::KeySymb;

use errno::Errno;

use crate::terminal::TERMINAL;

// static mut KEY_SYMB_OPT: Option<KeySymb> = None;

/// In a TTY keysymb control is up, handle it. This function must be called from a non-interruptble context
// pub unsafe fn handle_tty_control() {
//     if let Some(keysymb) = KEY_SYMB_OPT {
//         // Check if is not a special tty control before register character
//         if TERMINAL.as_mut().unwrap().handle_tty_control(keysymb) {
//             KEY_SYMB_OPT = None;
//         }
//     }
// }

// /// Get the stored character
// pub fn get_keysymb() -> Option<u32> {
//     unsafe { KEY_SYMB_OPT.map(|evt| evt as u32) }
// }

fn read_from_terminal(_fd: i32, buf: *mut u8, _count: usize) -> u32 {
    let mut keysymb_buf: [KeySymb; 1] = [KeySymb::nul; 1];
    let read_count = unsafe { TERMINAL.as_mut().unwrap().read(&mut keysymb_buf, 0) };
    if read_count != 0 {
        let ret = keysymb_buf[0];
        unsafe {
            // TODO: Fix this keysymb hack one day
            *buf = ret as u8;
            *buf.add(1) = ((ret as u32 & 0xff00) >> 8) as u8;
        }
    }
    return read_count as u32;
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

            let read_count = read_from_terminal(fd, buf, count);
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
                let read_count = read_from_terminal(fd, buf, count);

                if read_count == 0 {
                    panic!("read has been wake up but there is nothing to read");
                }
                return Ok(2);
            }
        } else {
            Err(Errno::Eperm)
        }
    })
}
