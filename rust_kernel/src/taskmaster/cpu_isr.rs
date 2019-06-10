//! this file countains code about cpu exception reassignement and use

use super::process::CpuState;
use super::scheduler::{SCHEDULER, SIGNAL_LOCK};
use super::signal::Signum;
use super::syscall::sys_signal::sys_kill;

use core::ffi::c_void;

use crate::interrupts::idt::{GateType, IdtGateEntry, InterruptTable};
use crate::memory::KERNEL_VIRTUAL_PAGE_ALLOCATOR;
use crate::panic::{get_page_fault_origin, qemu_check, trace_back};

extern "C" {
    fn _cpu_isr_divide_by_zero() -> !;
    fn _cpu_isr_debug() -> !;
    fn _cpu_isr_non_maskable_interrupt() -> !;
    fn _cpu_isr_breakpoint() -> !;
    fn _cpu_isr_overflow() -> !;
    fn _cpu_isr_bound_range_exceeded() -> !;
    fn _cpu_isr_invalid_opcode() -> !;
    fn _cpu_isr_no_device() -> !;
    fn _cpu_isr_double_fault() -> !;
    fn _cpu_isr_fpu_seg_overrun() -> !;
    fn _cpu_isr_invalid_tss() -> !;
    fn _cpu_isr_seg_no_present() -> !;
    fn _cpu_isr_stack_seg_fault() -> !;
    fn _cpu_isr_general_protect_fault() -> !;
    fn _cpu_isr_page_fault() -> !;
    // no.15 reserved
    fn _cpu_isr_fpu_floating_point_exep() -> !;
    fn _cpu_isr_alignment_check() -> !;
    fn _cpu_isr_machine_check() -> !;
    fn _cpu_isr_simd_fpu_fp_exception() -> !;
    fn _cpu_isr_virtualize_exception() -> !;
    // 21-29 reserved
    fn _cpu_isr_security_exception() -> !;
    // 31 reserved

    fn _read_cr2() -> u32;
}

extern "C" fn reserved_exception() -> ! {
    panic!("This is a reserved exception");
}

/// The list of the default exception handlers.
/// They are loaded by the `init_cpu_exceptions` method.
const CPU_EXCEPTIONS: [(unsafe extern "C" fn() -> !, &str, GateType); 32] = [
    (_cpu_isr_divide_by_zero, "division by zero", GateType::InterruptGate32),
    (_cpu_isr_debug, "debug", GateType::TrapGate32),
    (_cpu_isr_non_maskable_interrupt, "non-maskable interrupt", GateType::InterruptGate32),
    (_cpu_isr_breakpoint, "breakpoint", GateType::TrapGate32),
    (_cpu_isr_overflow, "overflow", GateType::TrapGate32),
    (_cpu_isr_bound_range_exceeded, "bound range exceeded", GateType::InterruptGate32),
    (_cpu_isr_invalid_opcode, "invalid opcode", GateType::InterruptGate32),
    (_cpu_isr_no_device, "no device", GateType::InterruptGate32),
    (_cpu_isr_double_fault, "double fault", GateType::InterruptGate32),
    (_cpu_isr_fpu_seg_overrun, "fpu seg overrun", GateType::InterruptGate32),
    (_cpu_isr_invalid_tss, "invalid tss", GateType::InterruptGate32),
    (_cpu_isr_seg_no_present, "seg no present", GateType::InterruptGate32),
    (_cpu_isr_stack_seg_fault, "stack seg fault", GateType::InterruptGate32),
    (_cpu_isr_general_protect_fault, "general protection fault", GateType::InterruptGate32),
    (_cpu_isr_page_fault, "page fault", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (_cpu_isr_fpu_floating_point_exep, "fpu floating point exception", GateType::InterruptGate32),
    (_cpu_isr_alignment_check, "alignement check", GateType::InterruptGate32),
    (_cpu_isr_machine_check, "machine check", GateType::InterruptGate32),
    (_cpu_isr_simd_fpu_fp_exception, "simd fpu exception", GateType::InterruptGate32),
    (_cpu_isr_virtualize_exception, "virtualize exception", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (_cpu_isr_security_exception, "security exception", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
];

/// Set the CPU exceptions vectors on the first 32 entries.
/// # Panics
/// Panics if the interruptions are not disabled when this is called, that is, if interrupts::get_interrupts_state() == true.
pub unsafe fn reassign_cpu_exceptions() {
    let mut interrupt_table = InterruptTable::current_interrupt_table().unwrap();

    let mut gate_entry = *IdtGateEntry::new()
        .set_storage_segment(false)
        .set_privilege_level(0)
        .set_selector(1 << 3)
        .set_gate_type(GateType::InterruptGate32);

    for (index, &(exception, _, gate_type)) in CPU_EXCEPTIONS.iter().enumerate() {
        gate_entry.set_handler(exception as *const c_void as u32).set_gate_type(gate_type);

        interrupt_table[index] = gate_entry;
    }
}

#[no_mangle]
unsafe extern "C" fn cpu_isr_interrupt_handler(cpu_state: *mut CpuState) {
    let cs = (*cpu_state).cs;
    // Error from ring 0
    if cs & 0b11 == 0 {
        // Handle kernel page fault
        if (*cpu_state).cpu_isr_reserved == 14 {
            let virtual_page_allocator = KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap();
            // Kernel valloc case
            if let Ok(()) = virtual_page_allocator.valloc_handle_page_fault(_read_cr2()) {
                return;
            } else {
                let page_fault_cause = get_page_fault_origin((*cpu_state).err_code_reserved);
                eprintln!("{}", page_fault_cause);
            }
        }
        eprintln!("Kernel Panic: {}\n{:X?}", CPU_EXCEPTIONS[(*cpu_state).cpu_isr_reserved as usize].1, *cpu_state);
        trace_back(((*cpu_state).eip, (*cpu_state).registers.ebp as *const u32));
        qemu_check();
        loop {}
    // Error from ring 3
    } else if cs & 0b11 == 0b11 {
        // Temporaly display a debug
        eprintln!("\n{:X?}", *cpu_state);
        eprintln!("Stack informations 'ss: 0x{:X?} esp: 0x{:X?}'", (*cpu_state).ss, (*cpu_state).esp);
        eprintln!("Cannot display backtrace from a non-kernel routine !");

        // Send a kill signum to the current process: kernel-sodo mode
        let current_task_pid = SCHEDULER.lock().current_task_pid();
        let _res = match (*cpu_state).cpu_isr_reserved {
            14 => sys_kill(current_task_pid, Signum::Sigsegv as u32),
            _ => {
                eprintln!("{}", CPU_EXCEPTIONS[(*cpu_state).cpu_isr_reserved as usize].1);
                sys_kill(current_task_pid, Signum::Sigkill as u32)
            }
        };

        // On ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
        if SIGNAL_LOCK == true {
            SIGNAL_LOCK = false;
            SCHEDULER.lock().current_task_apply_pending_signals(cpu_state as u32, false);
        }
        // TODO: Remove that later
        loop {}
    // Unknown ring
    } else {
        eprintln!("Stange CS value: 0x{:X?}. Cannot display more informations", cs);
        qemu_check();
        loop {}
    }
}
