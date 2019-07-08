//! This file contains the task manager

mod cpu_isr;
mod ipc;
mod process;
#[macro_use]
mod scheduler;
mod safe_ffi;
mod signal;
mod syscall;
mod task;
mod tests;
mod thread_group;

pub use process::{KernelProcess, Process, TaskOrigin, UserProcess};
use scheduler::SCHEDULER;

#[allow(unused)]
use tests::*;

use alloc::boxed::Box;
use alloc::vec::Vec;
use errno::Errno;

/// SysResult is just made to handle module errors. Return optional return and errno
pub type SysResult<T> = core::result::Result<T, Errno>;

/// MonoTasking or MultiTasking configuration
pub enum TaskMode {
    /// MonoTasking mode
    Mono,
    /// MultiTasking mode, param: frequency
    Multi(f32),
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
        SCHEDULER.lock().add_user_process(None, p).unwrap();
    }

    // Set the scheduler idle process
    SCHEDULER
        .lock()
        .set_idle_process(unsafe {
            KernelProcess::new(TaskOrigin::Raw(
                _idle_process_code as *const u8,
                _idle_process_len,
            ))
            .unwrap()
        })
        .unwrap();

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(20.)) }
}

extern "C" {
    fn _idle_process_code();
    static _idle_process_len: usize;
}
