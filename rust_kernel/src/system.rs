pub mod i8086_payload;
pub use i8086_payload::{i8086_payload, i8086_payload_apm_shutdown};

use i386::BaseRegisters;

use interrupts::idt::InterruptTable;
use interrupts::GateType;
use interrupts::GateType::{InterruptGate32, TrapGate32};

/// get the symbol addr
#[macro_use]
macro_rules! symbol_addr {
    ($ident: ident) => {
        #[allow(unused_unsafe)]
        unsafe {
            &$ident as *const _ as usize
        }
    };
}
pub struct Cr0;

#[no_mangle]
extern "C" {
    /// reg is the input parameter and the output
    fn _int8086(reg: *mut BaseRegisters, bios_int: u16) -> u16;
}

/// This is a wrapper of the _real_mode_op fonction.
/// It should be used instead of using _real_mode_op directly,
/// as it disable the interrupts and resets the PICs to there default
/// values before calling _real_mode_op.
/// It then restores the interrupts state and the PICs to there old IMR and vector offsets.

#[no_mangle]
pub unsafe fn real_mode_op(reg: *mut BaseRegisters, bios_int: u16) -> u16 {
    use crate::drivers::{pic_8259, Pic8259, PIC_8259};

    without_interrupts!({
        let ret;
        // check if PIC is initialized
        let mut pic_8259 = PIC_8259.lock();
        match pic_8259.is_initialized() {
            false => ret = _int8086(reg, bios_int),
            true => {
                let imrs = pic_8259.reset_to_default();

                ret = _int8086(reg, bios_int);

                let conf = Pic8259::basic_pic_configuration(
                    pic_8259::KERNEL_PIC_MASTER_IDT_VECTOR,
                    pic_8259::KERNEL_PIC_SLAVE_IDT_VECTOR,
                );
                pic_8259.initialize(conf);
                pic_8259.set_masks(imrs);
            }
        }
        ret
    })
}

/// This function initialize the Interrupt module: The default Idtr and InterruptTable are loaded,
/// then the PIC is configured.
/// This function returns the created InterruptTable.
pub unsafe fn init_idt<'a>() -> InterruptTable<'a> {
    interrupts::init(CPU_EXCEPTIONS, _default_isr)
}

/// The list of the default exception handlers.
/// They are loaded by the `init_cpu_exceptions` method.
const CPU_EXCEPTIONS: [(unsafe extern "C" fn() -> !, GateType); 32] = [
    (_isr_divide_by_zero, InterruptGate32),
    (_isr_debug, TrapGate32),
    (_isr_non_maskable_interrupt, InterruptGate32),
    (_isr_breakpoint, TrapGate32),
    (_isr_overflow, TrapGate32),
    (_isr_bound_range_exceeded, InterruptGate32),
    (_isr_invalid_opcode, InterruptGate32),
    (_isr_no_device, InterruptGate32),
    (_isr_double_fault, InterruptGate32),
    (_isr_fpu_seg_overrun, InterruptGate32),
    (_isr_invalid_tss, InterruptGate32),
    (_isr_seg_no_present, InterruptGate32),
    (_isr_stack_seg_fault, InterruptGate32),
    (_isr_general_protect_fault, InterruptGate32),
    (_isr_page_fault, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (_isr_fpu_floating_point_exep, InterruptGate32),
    (_isr_alignment_check, InterruptGate32),
    (_isr_machine_check, InterruptGate32),
    (_isr_simd_fpu_fp_exception, InterruptGate32),
    (_isr_virtualize_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
    (_isr_security_exception, InterruptGate32),
    (reserved_exception, InterruptGate32),
];

extern "C" {
    pub(super) fn _isr_divide_by_zero() -> !;
    pub(super) fn _isr_debug() -> !;
    pub(super) fn _isr_non_maskable_interrupt() -> !;
    pub(super) fn _isr_breakpoint() -> !;
    pub(super) fn _isr_overflow() -> !;
    pub(super) fn _isr_bound_range_exceeded() -> !;
    pub(super) fn _isr_invalid_opcode() -> !;
    pub(super) fn _isr_no_device() -> !;
    pub(super) fn _isr_double_fault() -> !;
    pub(super) fn _isr_fpu_seg_overrun() -> !;
    pub(super) fn _isr_invalid_tss() -> !;
    pub(super) fn _isr_seg_no_present() -> !;
    pub(super) fn _isr_stack_seg_fault() -> !;
    pub(super) fn _isr_general_protect_fault() -> !;
    pub(super) fn _isr_page_fault() -> !;
    // no.15 reserved
    pub(super) fn _isr_fpu_floating_point_exep() -> !;
    pub(super) fn _isr_alignment_check() -> !;
    pub(super) fn _isr_machine_check() -> !;
    pub(super) fn _isr_simd_fpu_fp_exception() -> !;
    pub(super) fn _isr_virtualize_exception() -> !;
    // 21-29 reserved
    pub(super) fn _isr_security_exception() -> !;
// 31 reserved
}

pub(super) extern "C" fn reserved_exception() -> ! {
    panic!("This is a reserved exception");
}

extern "C" {
    fn _default_isr();
}
