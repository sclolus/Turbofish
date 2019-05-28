//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;

use process::{tss::Tss, CpuState, Process, TaskOrigin};
use scheduler::SCHEDULER;
use tests::*;

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

    // Initialize the TSS segment (necessary for ring3 switch)
    let _t = unsafe { Tss::init(&kernel_stack as *const u8 as u32, 0x18) };
    Tss::display();

    // Create an ASM dummy process based on a simple function
    let p1 = unsafe { Process::new(TaskOrigin::Raw(&_dummy_asm_process_code, _dummy_asm_process_len)).unwrap() };
    println!("{:#X?}", p1);

    // Create a real rust process based on an ELF file
    let p2 = unsafe { Process::new(TaskOrigin::Elf(&include_bytes!("./vincent")[..])).unwrap() };
    println!("{:#X?}", p2);

    // Load some processes into the scheduler
    // SCHEDULER.lock().add_process(p1);
    SCHEDULER.lock().add_process(p2);

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Mono) }
}
