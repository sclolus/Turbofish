//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;

use process::{tss::Tss, CpuState, Process};
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

    // Create an entire C dummy process
    let p1 = unsafe { Process::new(&dummy_c_process, 4096) };
    println!("{:#X?}", p1);

    // Create an entire ASM dummy process
    let p2 = unsafe { Process::new(&_dummy_asm_process_code, _dummy_asm_process_len) };
    println!("{:#X?}", p2);

    // Create a real process
    let p3 = unsafe { Process::load().unwrap() };
    println!("{:#X?}", p3);

    // Load some processes into the scheduler
    // SCHEDULER.lock().add_process(p1);
    // SCHEDULER.lock().add_process(p2);
    SCHEDULER.lock().add_process(p3);

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Mono) }
}
