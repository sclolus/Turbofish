//! This file contains the task manager

mod process;
mod scheduler;
mod syscall;
mod tests;

use process::{tss::Tss, CpuState, Process, ProcessType};
use scheduler::SCHEDULER;
use tests::{rust_kernel_processes::*, *};

/// MonoTasking or MultiTasking configuration
pub enum TaskMode {
    Mono,
    Multi,
}

/// Main function of taskMaster Initialisation
pub fn start() -> ! {
    // Initialize Syscall system
    syscall::init();

    // Initialize the TSS segment (necessary for ring3 switch)
    let _t = unsafe { Tss::init(&kernel_stack as *const u8 as u32, 0x18) };
    Tss::display();

    // Create an entire C dummy process
    let p1 = unsafe { Process::new(&dummy_c_process, Some(4096), ProcessType::Ring3) };
    println!("{:#X?}", p1);

    // Create an entire ASM dummy process
    let p2 = unsafe { Process::new(&_dummy_asm_process_code, Some(_dummy_asm_process_len), ProcessType::Ring3) };
    println!("{:#X?}", p2);

    // Create an entire kernel dummy process who make a true syscall
    let p3 = unsafe { Process::new(process_zero as *const fn() as *const u8, None, ProcessType::Kernel) };
    println!("{:#X?}", p3);

    // Create an entire kernel dummy process who get the stack value
    let p4 = unsafe { Process::new(get_stack as *const fn() as *const u8, None, ProcessType::Kernel) };
    println!("{:#X?}", p4);

    // Create a entire kernel shell process
    let p5 = unsafe { Process::new(crate::shell::shell as *const fn() as *const u8, None, ProcessType::Kernel) };
    println!("{:#X?}", p5);

    // Load some processes into the scheduler
    SCHEDULER.lock().add_process(p1);
    SCHEDULER.lock().add_process(p2);

    // Launch the scheduler
    unsafe { scheduler::start(TaskMode::Multi) }
}
