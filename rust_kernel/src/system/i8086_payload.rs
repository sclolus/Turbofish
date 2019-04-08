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
        payload: *const extern "C" fn(some_stuff: u8, ...),
        size_fn: usize,
    ) -> i32;
}

extern "C" {
    pub static payload_apm_shutdown: extern "C" fn(some_stuff: u8, ...);
    pub static payload_apm_shutdown_len: usize;
}

pub extern "C" fn i8086_payload_apm_shutdown() -> I8086PayloadResult<()> {
    let mut reg: BaseRegisters = BaseRegisters { ..Default::default() };

    let ret =
        unsafe { _i8086_payload(&mut reg as *mut BaseRegisters, &payload_apm_shutdown, payload_apm_shutdown_len) };

    if ret == -1 {
        Err(I8086PayloadError::ApmShutdownFailure)
    } else {
        Ok(())
    }
}
