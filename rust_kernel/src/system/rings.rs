use core::convert::{From, Into};
    
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum PrivilegeLevel {
    // Most priviliged level, Most of the critical kernel code is expected to run at this level
    Ring0 = 0b00,

    // To be discussed.
    Ring1 = 0b01,

    // To be discussed.
    Ring2 = 0b10,

    // Normal user Privilege Level
    Ring3 = 0b11,
}

impl From<u8> for PrivilegeLevel {
    fn from(from: u8) -> Self {
        use PrivilegeLevel::*;
        
        match from {
            0b00 => Ring0,
            0b01 => Ring1,
            0b10 => Ring2,
            _ => Ring3,
        }
    }
}
