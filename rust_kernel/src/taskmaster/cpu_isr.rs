//! this file countains code about cpu exception reassignement and use

use super::process::CpuState;
use super::scheduler::{Scheduler, SCHEDULER};
use super::syscall::signalfn::sys_kill;
use libc_binding::Signum;

use core::ffi::c_void;
use elf_loader::SymbolTable;

use crate::interrupts::idt::{GateType, IdtGateEntry, InterruptTable};
use crate::memory::{AddressSpace, KERNEL_VIRTUAL_PAGE_ALLOCATOR};
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
    (
        _cpu_isr_divide_by_zero,
        "division by zero",
        GateType::InterruptGate32,
    ),
    (_cpu_isr_debug, "debug", GateType::TrapGate32),
    (
        _cpu_isr_non_maskable_interrupt,
        "non-maskable interrupt",
        GateType::InterruptGate32,
    ),
    (_cpu_isr_breakpoint, "breakpoint", GateType::TrapGate32),
    (_cpu_isr_overflow, "overflow", GateType::TrapGate32),
    (
        _cpu_isr_bound_range_exceeded,
        "bound range exceeded",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_invalid_opcode,
        "invalid opcode",
        GateType::InterruptGate32,
    ),
    (_cpu_isr_no_device, "no device", GateType::InterruptGate32),
    (
        _cpu_isr_double_fault,
        "double fault",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_fpu_seg_overrun,
        "fpu seg overrun",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_invalid_tss,
        "invalid tss",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_seg_no_present,
        "seg no present",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_stack_seg_fault,
        "stack seg fault",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_general_protect_fault,
        "general protection fault",
        GateType::InterruptGate32,
    ),
    (_cpu_isr_page_fault, "page fault", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (
        _cpu_isr_fpu_floating_point_exep,
        "fpu floating point exception",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_alignment_check,
        "alignement check",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_machine_check,
        "machine check",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_simd_fpu_fp_exception,
        "simd fpu exception",
        GateType::InterruptGate32,
    ),
    (
        _cpu_isr_virtualize_exception,
        "virtualize exception",
        GateType::InterruptGate32,
    ),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (reserved_exception, "reserved", GateType::InterruptGate32),
    (
        _cpu_isr_security_exception,
        "security exception",
        GateType::InterruptGate32,
    ),
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
        gate_entry
            .set_handler(exception as *const c_void as u32)
            .set_gate_type(gate_type);

        interrupt_table[index] = gate_entry;
    }
}

/// Get eip from ebp (return tupple of (eip, ebp))
fn get_eip(address_space: &AddressSpace, ebp: *const u32) -> Result<(u32, *const u32), ()> {
    // Check if pointer exists in user virtual address space
    address_space
        .check_user_ptr::<u32>(unsafe { ebp.add(1) })
        .map_err(|_| ())?;
    let eip = unsafe { *ebp.add(1) };
    if eip == 0 {
        Ok((0, ebp))
    } else {
        // Check if pointer exists in user virtual address space
        address_space.check_user_ptr::<u32>(ebp).map_err(|_| ())?;
        Ok((eip, unsafe { *ebp as *const u32 }))
    }
}

/// Take the first eip and epb as parameter and trace back up.
fn trace_process(
    address_space: &AddressSpace,
    symbol_table: &SymbolTable,
    mut s: (u32, *const u32),
) -> Result<(), ()> {
    loop {
        let symbol = symbol_table.get_symbol(s.0);
        match symbol {
            Some(sym) => log::info!("{:#X?} on ({:#X?} {:?})", s.0, sym.1, sym.0),
            // TODO: Stop considering as bullshit if there are no symbol name
            None => {
                log::info!("{:#X?} Unexpected Symbol", s.0);
                break;
            }
        }
        s = get_eip(address_space, s.1)?;
        if s.0 == 0 {
            break;
        }
    }
    Ok(())
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
                eprintln!("{}     address: {:#X?}", page_fault_cause, _read_cr2());
            }
        }
        eprintln!(
            "Kernel Panic: {}\n{:X?}",
            CPU_EXCEPTIONS[(*cpu_state).cpu_isr_reserved as usize].1,
            *cpu_state
        );
        trace_back(((*cpu_state).eip, (*cpu_state).registers.ebp as *const u32));
        qemu_check();
        loop {}
    // Error from ring 3
    } else if cs & 0b11 == 0b11 {
        // Temporaly display a debug
        let page_fault_cause = get_page_fault_origin((*cpu_state).err_code_reserved);
        log::warn!("{}     address: {:#X?}", page_fault_cause, _read_cr2());
        log::warn!("{:X?}", *cpu_state);
        log::warn!(
            "Stack informations 'ss: 0x{:X?} esp: 0x{:X?}'",
            (*cpu_state).ss,
            (*cpu_state).esp
        );

        {
            let scheduler = SCHEDULER.lock();

            let task = scheduler.current_task();
            let address_space = &task.unwrap_process().get_virtual_allocator();

            // Attempt to display the process backtrace
            match &task.unwrap_process().symbol_table {
                Some(symbol_table) => {
                    let _r = trace_process(
                        address_space,
                        symbol_table,
                        ((*cpu_state).eip, (*cpu_state).registers.ebp as *const u32),
                    )
                    .map_err(|e| {
                        log::warn!("Unexpected memory location founded in address space !");
                        e
                    });
                }
                None => log::warn!("Cannot trace a non-kernel process without his symbol list !"),
            }
        }

        // Send a kill signum to the current process: kernel-sodo mode
        let current_task_pid = SCHEDULER.lock().current_task_id().0;
        let _res = match (*cpu_state).cpu_isr_reserved {
            14 => sys_kill(current_task_pid as i32, Signum::SIGSEGV as u32),
            _ => {
                log::warn!(
                    "{}",
                    CPU_EXCEPTIONS[(*cpu_state).cpu_isr_reserved as usize].1
                );
                sys_kill(current_task_pid as i32, Signum::SIGKILL as u32)
            }
        };

        // On ring3 process -> Mark process on signal execution state, modify CPU state, prepare a signal frame.
        SCHEDULER
            .lock()
            .current_task_deliver_pending_signals(cpu_state, Scheduler::NOT_IN_BLOCKED_SYSCALL);
    // Unknown ring
    } else {
        eprintln!(
            "Stange CS value: 0x{:X?}. Cannot display more informations",
            cs
        );
        qemu_check();
        loop {}
    }
}
