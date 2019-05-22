pub mod rings;
pub use rings::PrivilegeLevel;

pub mod i8086_payload;
pub use i8086_payload::{i8086_payload, i8086_payload_apm_shutdown};

pub use registers::{BaseRegisters, ExtendedRegisters};

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
