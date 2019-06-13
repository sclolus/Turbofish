//! sys_read()

use super::SysResult;

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, unpreemptible};
use super::task::WaitingState;

use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};

use errno::Errno;

use crate::terminal::TERMINAL;

// TODO: Fix nasty processes concurence
static mut KEY_SYMB_OPT: Option<KeySymb> = None;

/// Usefull method to stock the character from the keyboard
pub fn stock_keysymb(keysymb: KeySymb) {
    unsafe {
        // Check if is not a special tty control before register character
        if !TERMINAL.as_mut().unwrap().handle_tty_control(keysymb) {
            KEY_SYMB_OPT = Some(keysymb);
        }
    }
}

/// Get the stored character
pub fn get_keysymb() -> Option<u32> {
    unsafe { KEY_SYMB_OPT.map(|evt| evt as u32) }
}

/// Read something from a file descriptor
pub fn sys_read(fd: i32, buf: *mut u8, count: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = &mut scheduler.current_task_mut().unwrap_process_mut().virtual_allocator;

        // Check if pointer exists in user virtual address space
        v.check_user_ptr_with_len::<u8>(buf, count)?;

        if fd == 0 {
            // Auto-preempt calling
            unsafe {
                KEY_SYMB_OPT = None;
                // Register callback
                KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
            }

            scheduler.current_task_mut().set_waiting(WaitingState::Event(get_keysymb));

            let ret = auto_preempt();

            unpreemptible();

            if ret < 0 {
                return Err(Errno::Eintr);
            } else {
                // TODO: May be more bigger. TODO: Check size
                // TODO: Must be sizeof of readen character
                unsafe {
                    *buf = ret as u8;
                }
                return Ok(1);
            }
        } else {
            Err(Errno::Eperm)
        }
    })
}
