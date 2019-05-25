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

    // Create an ASM dummy process based on a simple function
    let p1 = unsafe { Process::new(TaskOrigin::Raw(&_dummy_asm_process_code, _dummy_asm_process_len)).unwrap() };
    println!("{:#X?}", p1);

    // Create a real rust process based on an ELF file
    let p2 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap() };
    println!("{:#X?}", p2);

    // Create a real rust process based on an ELF file
    let p3 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap() };
    println!("{:#X?}", p3);

    // Create a real rust process based on an ELF file
    let p4 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap() };
    println!("{:#X?}", p4);

    // Create a real rust process based on an ELF file
    let p5 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    println!("{:#X?}", p5);

    // Create a real rust process based on an ELF file
    let p6 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    println!("{:#X?}", p6);

    // Create a real rust process based on an ELF file
    let p7 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap() };
    println!("{:#X?}", p7);

    // Load some processes into the scheduler
    // SCHEDULER.lock().add_process(p1);
    SCHEDULER.lock().add_process(p2);
    SCHEDULER.lock().add_process(p3);
    SCHEDULER.lock().add_process(p4);
    // SCHEDULER.lock().add_process(p5);
    // SCHEDULER.lock().add_process(p6);
    // SCHEDULER.lock().add_process(p7);

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi(20.)) }
}
