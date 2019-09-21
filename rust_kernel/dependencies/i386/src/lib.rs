#![cfg_attr(not(test), no_std)]
#![feature(stdsimd)]
#![feature(asm)]

use bit_field::BitField;

use core::fmt;
use core::fmt::Display;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[derive(Default)]
pub struct BaseRegisters {
    /*0        |*/ pub edi: u32,
    /*4        |*/ pub esi: u32,
    /*8        |*/ pub ebp: u32,
    /*12       |*/ pub esp: u32,
    /*16       |*/ pub ebx: u32,
    /*20       |*/ pub edx: u32,
    /*24       |*/ pub ecx: u32,
    /*28       |*/ pub eax: u32,
    /*32       |*/
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtendedRegisters {
    /*0       |*/ pub ds: u32,
    /*4       |*/ pub es: u32,
    /*8       |*/ pub fs: u32,
    /*12      |*/ pub gs: u32,
    /*16      |*/ pub ss: u32,
    /*20      |*/ pub eip: u32,
    /*24      |*/ pub cs: u32,
    /*28      |*/ pub eflags: u32,
    /*32      |*/ pub edi: u32,
    /*36      |*/ pub esi: u32,
    /*40      |*/ pub new_ebp: u32,
    /*44      |*/ pub esp: u32,
    /*48      |*/ pub ebx: u32,
    /*52      |*/ pub edx: u32,
    /*56      |*/ pub ecx: u32,
    /*60      |*/ pub eax: u32,
    /*64      |*/ pub old_ebp: u32,
    /*68      |*/
}

impl core::fmt::Debug for ExtendedRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "cs: {:#X?}, ds: {:#X?}, es: {:#X?}, fs: {:#X?}, gs: {:#X?}, ss: {:#X?}\n\
             esi: {:#010X?}, edi: {:#010X?}, ebp: {:#010X?}, esp: {:#010X?}\n\
             eax: {:#010X?}, ebx: {:#010X?}, ecx: {:#010X?}, edx: {:#010X?}\n\
             eip: {:#010X?}, eflags => {:#010X?}",
            self.cs,
            self.ds,
            self.es,
            self.fs,
            self.gs,
            self.ss,
            self.esi,
            self.edi,
            self.new_ebp,
            self.esp,
            self.eax,
            self.ebx,
            self.ecx,
            self.edx,
            self.eip,
            self.eflags
        )
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[repr(C)]
/// The content of the EFLAGS register at the moment it was fetched.
/// WARNING: The flags in themself are not guaranted not to have changed since the structure was fetched.
/// This is merely a snapshot of the flags at a given moment.
pub struct Eflags {
    inner: u32,
}

impl Eflags {
    pub fn new(inner: u32) -> Self {
        Self { inner }
    }
    pub fn inner(&self) -> u32 {
        self.inner
    }
    #[cfg(test)]
    /// Placeholder for get_eflags in 64-bit mode. Panics.
    pub fn get_eflags() -> Self {
        panic!("Get_eflags should not be called in 64-bit mode (until we supported x86_64)");
    }

    /// Gets the current EFLAGS
    #[cfg(not(test))]
    pub fn get_eflags() -> Self {
        let inner: u32;

        unsafe {
            asm!("pushfd
                       pop eax"
                      : "={eax}" (inner) : : : "intel", "volatile")
        };
        Eflags { inner }
    }

    /// Returns the state of the carry flag.
    pub fn carry_flag(&self) -> bool {
        self.inner.get_bit(0)
    }

    /// Returns the state of the parity flag.
    pub fn parity_flag(&self) -> bool {
        self.inner.get_bit(2)
    }

    /// Returns the state of the adjust flag.
    pub fn adjust_flag(&self) -> bool {
        self.inner.get_bit(4)
    }

    /// Returns the state of the zero flag.
    pub fn zero_flag(&self) -> bool {
        self.inner.get_bit(6)
    }

    /// Returns the state of the sign flag.
    pub fn sign_flag(&self) -> bool {
        self.inner.get_bit(7)
    }

    /// Returns the state of the trap flag.
    pub fn trap_flag(&self) -> bool {
        self.inner.get_bit(8)
    }

    /// Returns the state of the interrupt flag.
    pub fn interrupt_flag(&self) -> bool {
        self.inner.get_bit(9)
    }

    /// set the state of the interrupt flag.
    pub fn set_interrupt_flag(&mut self, value: bool) -> Self {
        self.inner.set_bit(9, value);
        *self
    }

    /// Returns the state of the direction.
    pub fn direction_flag(&self) -> bool {
        self.inner.get_bit(10)
    }

    /// Returns the state of the overflow flag.
    pub fn overflow_flag(&self) -> bool {
        self.inner.get_bit(11)
    }

    /// Returns the current I/O privilege level.
    pub fn iopl(&self) -> u8 {
        self.inner.get_bits(12..14) as u8
    }

    /// Returns the state of the nested task flag.
    pub fn nested_task_flag(&self) -> bool {
        self.inner.get_bit(14)
    }

    /// Returns the state of the resume flag.
    pub fn resume_flag(&self) -> bool {
        self.inner.get_bit(16)
    }

    /// Returns the state of the virtual 8086 mode flag.
    pub fn virtual_8086_mode_flag(&self) -> bool {
        self.inner.get_bit(17)
    }

    /// Returns the state of the alignment check flag.
    pub fn alignment_check_flag(&self) -> bool {
        self.inner.get_bit(18)
    }

    /// Returns the state of the virtual interrupt flag.
    pub fn virtual_interrupt_flag(&self) -> bool {
        self.inner.get_bit(19)
    }

    /// Returns if a virtual interrupt is pending.
    pub fn virtual_interrupt_pending(&self) -> bool {
        self.inner.get_bit(20)
    }

    /// Returns if the cpu supports the cpuid instruction.
    /// This method relies directly on core. So I'm not sure this should be a EFLAGS method, but it still has semantic sense.
    pub fn cpuid_flag(&self) -> bool {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::has_cpuid;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::has_cpuid;
        has_cpuid()
    }
}

impl Display for Eflags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "carry flag (CF): {}
parity flag (PF): {}
adjust flag (AF): {}
zero flag (ZF): {}
sign flag (SF): {}
trap flag (TF): {}
interrupt flag (IF): {}
direction flag (DF): {}
overflow flag (OF): {}
I/O privilege level (IOPL): {}
nested task flag (NT): {}
resume flag (RF): {}
virtual 8086 flag (VM): {}
alignment check flag (AC): {}
virtual interrupt flag (VIF): {}
virtual interrupt pending (VIP): {}
has CPUID (ID): {}\n",
            self.carry_flag(),
            self.parity_flag(),
            self.adjust_flag(),
            self.zero_flag(),
            self.sign_flag(),
            self.trap_flag(),
            self.interrupt_flag(),
            self.direction_flag(),
            self.overflow_flag(),
            self.iopl(),
            self.nested_task_flag(),
            self.resume_flag(),
            self.virtual_8086_mode_flag(),
            self.alignment_check_flag(),
            self.virtual_interrupt_flag(),
            self.virtual_interrupt_pending(),
            self.cpuid_flag()
        )
    }
}
use core::convert::From;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PrivilegeLevel {
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
