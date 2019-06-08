//! this file countains code about cpu exception reassignement and use

use super::process::CpuState;
use super::scheduler::{SCHEDULER, SIGNAL_LOCK};
use super::signal::{SignalStatus, Signum};
use super::syscall::signalfn::sys_kill;

use core::ffi::c_void;

use crate::interrupts::idt::{GateType, IdtGateEntry, InterruptTable};
use crate::panic::{qemu_check, trace_back};

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
}

extern "C" fn reserved_exception() -> ! {
    panic!("This is a reserved exception");
}

/// The list of the default exception handlers.
/// They are loaded by the `init_cpu_exceptions` method.
const CPU_EXCEPTIONS: [(unsafe extern "C" fn() -> !, GateType); 32] = [
    (_cpu_isr_divide_by_zero, GateType::InterruptGate32),
    (_cpu_isr_debug, GateType::TrapGate32),
    (_cpu_isr_non_maskable_interrupt, GateType::InterruptGate32),
    (_cpu_isr_breakpoint, GateType::TrapGate32),
    (_cpu_isr_overflow, GateType::TrapGate32),
    (_cpu_isr_bound_range_exceeded, GateType::InterruptGate32),
    (_cpu_isr_invalid_opcode, GateType::InterruptGate32),
    (_cpu_isr_no_device, GateType::InterruptGate32),
    (_cpu_isr_double_fault, GateType::InterruptGate32),
    (_cpu_isr_fpu_seg_overrun, GateType::InterruptGate32),
    (_cpu_isr_invalid_tss, GateType::InterruptGate32),
    (_cpu_isr_seg_no_present, GateType::InterruptGate32),
    (_cpu_isr_stack_seg_fault, GateType::InterruptGate32),
    (_cpu_isr_general_protect_fault, GateType::InterruptGate32),
    (_cpu_isr_page_fault, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (_cpu_isr_fpu_floating_point_exep, GateType::InterruptGate32),
    (_cpu_isr_alignment_check, GateType::InterruptGate32),
    (_cpu_isr_machine_check, GateType::InterruptGate32),
    (_cpu_isr_simd_fpu_fp_exception, GateType::InterruptGate32),
    (_cpu_isr_virtualize_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
    (_cpu_isr_security_exception, GateType::InterruptGate32),
    (reserved_exception, GateType::InterruptGate32),
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

    for (index, &(exception, gate_type)) in CPU_EXCEPTIONS.iter().enumerate() {
        gate_entry.set_handler(exception as *const c_void as u32).set_gate_type(gate_type);

        interrupt_table[index] = gate_entry;
    }
}

#[no_mangle]
unsafe extern "C" fn cpu_isr_interrupt_handler(cpu_state: *mut CpuState) {
    let cs = (*cpu_state).cs;
    if cs & 0b11 == 0 {
        eprintln!("\nKernel Panic:\n{:X?}", *cpu_state);
        trace_back(((*cpu_state).eip, (*cpu_state).registers.ebp as *const u32));
        qemu_check();
        loop {}
    } else if cs & 0b11 == 0b11 {
        // Temporaly display a debug
        eprintln!("\n{:X?}", *cpu_state);
        eprintln!("Stack informations 'ss: 0x{:X?} esp: 0x{:X?}'", (*cpu_state).ss, (*cpu_state).esp);
        eprintln!("Cannot display backtrace from a non-kernel routine !");

        // Send a kill signum to the current process: kernel-sodo mode
        let curr_process_pid = SCHEDULER.lock().curr_process_pid();
        let _res = match (*cpu_state).cpu_isr_reserved {
            14 => {
                eprintln!("Segmentation fault");
                sys_kill(curr_process_pid, Signum::Sigsegv as u32)
            }
            _ => {
                eprintln!("Illegal behavior");
                sys_kill(curr_process_pid, Signum::Sigkill as u32)
            }
        };

        // On ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
        if SIGNAL_LOCK == true {
            SIGNAL_LOCK = false;
            let signal = SCHEDULER.lock().curr_process_mut().signal.apply_pending_signals(cpu_state as u32);
            if let Some(SignalStatus::Deadly(signum)) = signal {
                SCHEDULER.lock().exit(signum as i32 * -1);
            }
        }
        // TODO: Remove that later
        loop {}
    } else {
        eprintln!("Stange CS value: 0x{:X?}. Cannot diaplay more informations", cs);
        qemu_check();
        loop {}
    }
}
