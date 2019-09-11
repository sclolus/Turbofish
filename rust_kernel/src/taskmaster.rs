//! This file contains the task manager

mod cpu_isr;
// mod ipc;
mod process;
#[macro_use]
mod scheduler;
pub mod drivers;
pub use drivers::{Driver, FileOperation};
mod fd_interface;
mod safe_ffi;
mod signal_interface;
mod syscall;
mod tests;
mod thread;
mod thread_group;
pub mod vfs;
pub use vfs::VFS;

use core::convert::{TryFrom, TryInto};
use thread_group::Credentials;
use vfs::Path;

mod sync;

/// Describe what to do after an IPC request and result return
#[derive(Debug)]
pub enum IpcResult<T> {
    /// Can continue thread execution normally
    Done(T),
    /// the user should wait for his IPC request
    Wait(T, usize),
}

impl<T> IpcResult<T> {
    pub fn expect(self, s: &'static str) -> T {
        match self {
            IpcResult::Done(t) => t,
            IpcResult::Wait(_, _) => panic!(s),
        }
    }
}

pub use process::{
    get_file_content, KernelProcess, Process, ProcessArguments, ProcessOrigin, UserProcess,
};
pub use safe_ffi::{CString, CStringArray};

use scheduler::SCHEDULER;

#[allow(unused)]
use tests::*;

use libc_binding::Errno;
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
pub fn start(filename: &str, argv: &[&str], envp: &[&str]) -> ! {
    // Reassign all cpu exceptions for taskmaster
    unsafe {
        cpu_isr::reassign_cpu_exceptions();
    }

    // Initialize Syscall system
    syscall::init();

    // Initialize VFS
    lazy_static::initialize(&VFS);

    // Register the first process
    let path = filename
        .try_into()
        .expect("The path of the init program is not valid");
    let file = get_file_content(&Path::try_from("/").unwrap(), &Credentials::ROOT, path)
        .expect("Cannot syncing");
    SCHEDULER
        .lock()
        .add_user_process(
            1,
            unsafe {
                UserProcess::new(
                    ProcessOrigin::Elf(&file),
                    Some(ProcessArguments::new(
                        argv.try_into().expect("argv creation failed"),
                        envp.try_into().expect("envp creation failed"),
                    )),
                )
            }
            .expect("Unexpected error when parsing ELF file"),
        )
        .expect("Scheduler is bullshit");

    // Register the keyboard callback
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
            KernelProcess::new(
                ProcessOrigin::Raw(_idle_process_code as *const u8, _idle_process_len),
                None,
            )
            .expect("Unexpected error while creating idle process")
        })
        .expect("Scheduler is bullshit");

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(1000.)) }
}

extern "C" {
    fn _idle_process_code();
    static _idle_process_len: usize;
}
