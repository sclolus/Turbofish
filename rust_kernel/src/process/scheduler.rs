use crate::memory::allocator::VirtualPageAllocator;
use crate::process::Process;
use crate::registers::Eflags;
use crate::system::BaseRegisters;
use alloc::vec;
use alloc::vec::Vec;

extern "C" {
    fn _process_a();
    fn _process_b();
    fn _switch_process(eflags: u32, segment: u32, eip: u32, esp: u32, registers: BaseRegisters) -> !;
    static process_a_stack: u8;
    static process_b_stack: u8;
    static kernel_stack: u8;
}

static mut PROCESS: Option<Vec<Process>> = None;
static mut CURR_PROCESS_INDEX: Option<usize> = None;

unsafe fn switch_process(next_process_index: usize) -> ! {
    let next_process: &Process = &PROCESS.as_mut().unwrap()[next_process_index];
    eprintln!("switch to eip:{:X?} esp:{:X?} reg:{:X?}", next_process.eip, next_process.esp, next_process.registers);

    next_process.virtual_allocator.context_switch();
    CURR_PROCESS_INDEX = Some(next_process_index);
    _switch_process(
        next_process.eflags.inner(),
        next_process.segment,
        next_process.eip,
        next_process.esp,
        next_process.registers,
    );
}

#[no_mangle]
unsafe extern "C" fn timer_interrupt_handler(
    old_eip: u32,
    old_segment: u32,
    old_eflags: u32,
    old_esp: u32,
    registers: BaseRegisters,
) -> ! {
    // eprintln!("kernel_stack: {:?}\n", &kernel_stack as *const u8);
    let eflags = crate::registers::Eflags::new(old_eflags);
    // eprintln!("bonjour segment: {:X}, eflags: {}", old_segment, eflags);

    eprintln!("sched");
    eprintln!("bonjour eip:{:X?} esp:{:X?} reg:{:X?}", old_eip, old_esp, registers);
    if !PROCESS.is_some() {
        PROCESS = Some(vec![
            Process::new(_process_a, Eflags::new(old_eflags)),
            Process::new(_process_b, Eflags::new(old_eflags)),
        ]);
    }
    match CURR_PROCESS_INDEX {
        None => {
            CURR_PROCESS_INDEX = Some(0);
            return switch_process(0);
        }
        Some(i) => {
            let next_process = if i == 0 { 1 } else { 0 };
            let curr_process: &mut Process = &mut PROCESS.as_mut().unwrap()[i];
            curr_process.eip = old_eip;
            curr_process.esp = old_esp;
            curr_process.registers = registers;
            curr_process.eflags = Eflags::new(old_eflags);
            curr_process.segment = old_segment;
            return switch_process(next_process);
        }
    }
}

#[no_mangle]
extern "C" fn process_a() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process A {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn process_b() {
    unsafe {
        for i in 0..1000000 {
            user_eprintln!("process B {}", i);
            asm!("hlt"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C" fn debug_process(eip: u32) {
    eprintln!("jump to eip |{:X}|\n", eip);
    crate::tests::helpers::exit_qemu(0);
    loop {}
}
