#![cfg_attr(not(test), no_std)]

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
