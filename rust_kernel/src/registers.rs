use bit_field::BitField;
use core::{fmt, fmt::Display};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
/// The content of the EFLAGS register at the moment it was fetched.
/// WARNING: The flags in themself are not guaranted not to have changed since the structure was fetched.
/// This is merely a snapshot of the flags at a given moment.
pub struct Eflags {
    inner: u32,
}

impl Eflags {
    /// Gets the current EFLAGS
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
    /// For unknown reasons, even if the cpu does support CPUID instruction, this flags still is false...
    pub fn cpuid_flag(&self) -> bool {
        self.inner.get_bit(21)
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

#[no_mangle]
extern "C" {
    pub fn _real_mode_op(reg: BaseRegisters, bios_int: u16) -> u16;
}

/// This is a wrapper of the _real_mode_op fonction.
/// It should be used instead of using _real_mode_op directly,
/// as it disable the interrupts and resets the PICs to there default
/// values before calling _real_mode_op.
/// It then restores the interrupts state and the PICs to there old IMR and vector offsets.
pub unsafe fn real_mode_op(reg: BaseRegisters, bios_int: u16) -> u16 {
    use crate::interrupts;
    use crate::interrupts::pic_8259;

    let interrupts_state = interrupts::get_interrupts_state();
    interrupts::disable();

    let imrs = pic_8259::reset_to_default();

    let ret = _real_mode_op(reg, bios_int);
    pic_8259::initialize(0x20, 0x28);

    pic_8259::restore_masks(imrs);
    interrupts::restore_interrupts_state(interrupts_state);
    ret
}
