use super::BaseRegisters;

/// Basics ACPI errors
#[derive(Copy, Clone, Debug)]
pub enum I8086PayloadError {
    ApmShutdownFailure,
}

/// Standard ACPI type result
pub type I8086PayloadResult<T> = core::result::Result<T, I8086PayloadError>;

extern "C" {
    fn _i8086_payload(
        reg: *mut BaseRegisters,
        payload: *const extern "C" fn(),
        size_fn: usize,
    ) -> i32;
}

/// This is a wrapper of the _i8086_payload fonction.
/// It should be used instead of using _i8086_payload directly,
/// as it disable the interrupts and resets the PICs to there default
/// values before calling _i8086_payload.
/// It then restores the interrupts state and the PICs to there old IMR and vector offsets.
pub unsafe fn i8086_payload(
    reg: *mut BaseRegisters,
    payload: *const extern "C" fn(),
    size_fn: usize,
) -> i32 {
    use crate::drivers::{pic_8259, PIC_8259};

    without_interrupts!({
        let ret;
        // check if PIC is initialized
        let mut pic_8259 = PIC_8259.lock();
        match pic_8259.is_initialized() {
            false => ret = _i8086_payload(reg, payload, size_fn),
            true => {
                let imrs = pic_8259.reset_to_default();

                ret = _i8086_payload(reg, payload, size_fn);

                pic_8259.set_idt_vectors(
                    pic_8259::KERNEL_PIC_MASTER_IDT_VECTOR,
                    pic_8259::KERNEL_PIC_SLAVE_IDT_VECTOR,
                );
                pic_8259.set_masks(imrs);
            }
        }
        ret
    })
}

extern "C" {
    static payload_apm_shutdown: extern "C" fn();
    static payload_apm_shutdown_len: usize;
}

pub extern "C" fn i8086_payload_apm_shutdown() -> I8086PayloadResult<()> {
    let mut reg: BaseRegisters = BaseRegisters {
        ..Default::default()
    };

    let ret = unsafe {
        i8086_payload(
            &mut reg as *mut BaseRegisters,
            &payload_apm_shutdown,
            payload_apm_shutdown_len,
        )
    };

    if ret == -1 {
        Err(I8086PayloadError::ApmShutdownFailure)
    } else {
        Ok(())
    }
}
