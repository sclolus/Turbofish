//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;
mod tools;

use process::{CpuState, KernelProcess, Process, TaskOrigin, UserProcess};
use scheduler::SCHEDULER;
use scheduler::{interruptible, uninterruptible};

#[allow(unused)]
use tests::*;

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
pub fn start() -> ! {
    // Initialize Syscall system
    syscall::init();

    // Load some processes into the scheduler
    let user_process_list = unsafe {
        vec![
            // UserProcess::new(TaskOrigin::Raw(&_dummy_asm_process_code_a, _dummy_asm_process_len_a)).unwrap(),
            // UserProcess::new(TaskOrigin::Raw(&_dummy_asm_process_code_b, _dummy_asm_process_len_b)).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/stack_overflow")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/sys_stack_overflow")[..])).unwrap(),
            UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_bomb")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Wait")[..])).unwrap(),
        ]
    };
    for (i, p) in user_process_list.into_iter().enumerate() {
        println!("user pocess no: {} : {:?}", i, p);
        SCHEDULER.lock().add_user_process(None, p).unwrap();
    }

    // Set the scheduler idle process
    SCHEDULER
        .lock()
        .set_idle_process(unsafe {
            KernelProcess::new(TaskOrigin::Raw(&_idle_process_code as *const _ as *const u8, _idle_process_len))
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
