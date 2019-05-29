//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;

use process::{CpuState, Process, TaskOrigin};
use scheduler::SCHEDULER;
use tests::{_dummy_asm_process_code, _dummy_asm_process_len};

use errno::Errno;

/// SysResult is just made to handle module errors
pub type SysResult<T> = core::result::Result<T, Errno>;

/// MonoTasking or MultiTasking configuration
pub enum TaskMode {
    /// MonoTasking mode
    Mono,
    /// MultiTasking mode, param: frequency
    Multi(f32),
}

/// Main function of taskMaster Initialisation
pub fn start() -> ! {
    // Initialize Syscall system
    syscall::init();

    let _p1 = unsafe { Process::new(TaskOrigin::Raw(&_dummy_asm_process_code, _dummy_asm_process_len)).unwrap() };
    let _p2 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap() };
    let _p3 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap() };
    let _p4 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap() };
    let _p5 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap() };
    let _p6 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    let _p7 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    let _p8 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    let _p9 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap() };
    let _p10 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/stack_overflow")[..])).unwrap() };
    let _p11 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/sys_stack_overflow")[..])).unwrap() };
    let _p13 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_bomb")[..])).unwrap() };

    // Load some processes into the scheduler
    // SCHEDULER.lock().add_process(_p1).unwrap();
    // SCHEDULER.lock().add_process(_p2).unwrap();
    // SCHEDULER.lock().add_process(_p3).unwrap();
    // SCHEDULER.lock().add_process(_p4).unwrap();
    // SCHEDULER.lock().add_process(_p5).unwrap();
    // SCHEDULER.lock().add_process(_p6).unwrap();
    // SCHEDULER.lock().add_process(_p7).unwrap();
    // SCHEDULER.lock().add_process(_p8).unwrap();
    // SCHEDULER.lock().add_process(_p9).unwrap();
    // SCHEDULER.lock().add_process(_p10).unwrap();
    // SCHEDULER.lock().add_process(_p11).unwrap();
    // SCHEDULER.lock().add_process(_p12).unwrap();
    SCHEDULER.lock().add_process(_p13).unwrap();

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(20.)) }
}
