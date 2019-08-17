//! This file contains the task manager

mod cpu_isr;
mod ipc;
mod process;
#[macro_use]
mod scheduler;
mod safe_ffi;
mod signal_interface;
mod syscall;
mod tests;
mod thread;
mod thread_group;

pub use process::{KernelProcess, Process, ProcessOrigin, UserProcess};
use scheduler::SCHEDULER;

#[allow(unused)]
use tests::*;

use alloc::boxed::Box;
use alloc::vec::Vec;
use errno::Errno;
use messaging::MessageTo;

/// SysResult is just made to handle module errors. Return optional return and errno
pub type SysResult<T> = core::result::Result<T, Errno>;

pub trait IntoRawResult {
    fn into_raw_result(self) -> u32;
}

impl IntoRawResult for SysResult<u32> {
    fn into_raw_result(self) -> u32 {
        match self {
            Ok(return_value) => return_value as u32,
            Err(errno) => (-(errno as i32)) as u32,
        }
    }
}

/// MonoTasking or MultiTasking configuration
pub enum TaskMode {
    /// MonoTasking mode
    Mono,
    /// MultiTasking mode, param: frequency
    Multi(f32),
}

use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};

/// we send a message
pub fn handle_key_press(key_pressed: KeySymb) {
    // in the keyboard interrupt handler, after reading the keysymb,
    // we send a message to the tty which will be handled in the next
    // schedule

    messaging::push_message(MessageTo::Tty { key_pressed })
}

// Create an ASM dummy process based on a simple function
/// Main function of taskMaster Initialisation
pub fn start(user_process_list: Vec<Box<UserProcess>>) -> ! {
    // Reassign all cpu exceptions for taskmaster
    unsafe {
        cpu_isr::reassign_cpu_exceptions();
    }

    // Initialize Syscall system
    syscall::init();
    for (_i, p) in user_process_list.into_iter().enumerate() {
        // println!("user pocess no: {} : {:?}", i, p);
        // all starting process inherit init for now, so their parent is 1
        SCHEDULER.lock().add_user_process(1, p).unwrap();
    }

    unsafe {
        KEYBOARD_DRIVER
            .as_mut()
            .unwrap()
            .bind(CallbackKeyboard::RequestKeySymb(handle_key_press));
    }
    // Set the scheduler idle process
    SCHEDULER
        .lock()
        .set_idle_process(unsafe {
            KernelProcess::new(ProcessOrigin::Raw(
                _idle_process_code as *const u8,
                _idle_process_len,
            ))
            .unwrap()
        })
        .unwrap();

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(1000.)) }
}

extern "C" {
    fn _idle_process_code();
    static _idle_process_len: usize;
}
