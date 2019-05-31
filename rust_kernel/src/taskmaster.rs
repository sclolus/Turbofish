//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;

use process::{CpuState, Process, TaskOrigin};
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
    let process_list = unsafe {
        vec![
            // Process::new(TaskOrigin::Raw(&_dummy_asm_process_code_a, _dummy_asm_process_len_a)).unwrap(),
            // Process::new(TaskOrigin::Raw(&_dummy_asm_process_code_b, _dummy_asm_process_len_b)).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/stack_overflow")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/sys_stack_overflow")[..])).unwrap(),
            Process::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_bomb")[..])).unwrap(),
            // Process::new(TaskOrigin::Elf(&include_bytes!("userland/Wait")[..])).unwrap(),
        ]
    };
    for (i, p) in process_list.into_iter().enumerate() {
        println!("pocess no: {} : {:?}", i, p);
        SCHEDULER.lock().add_process(None, p).unwrap();
    }

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(20.)) }
}
