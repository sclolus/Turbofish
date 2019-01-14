#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[repr(packed)]
#[derive(Default)]
pub struct BaseRegisters {
    /*0        |*/ pub edi:u32,
    /*4        |*/ pub esi:u32,
    /*8        |*/ pub ebp:u32,
    /*12       |*/ pub esp:u32,
    /*16       |*/ pub ebx:u32,
    /*20       |*/ pub edx:u32,
    /*24       |*/ pub ecx:u32,
    /*28       |*/ pub eax:u32,
    /*32       |*/ 
}

extern "C" {
    fn asm_real_mode_op
        (eax:u32, ebx:u32, ecx:u32, edx:u32, esi:u32, edi:u32, bios_int:u16) -> u32;
}

pub fn real_mode_op(reg: BaseRegisters, bios_interrupt: u16) -> u32 {
    unsafe {
        asm_real_mode_op(reg.eax, reg.ebx, reg.ecx, reg.edx, reg.esi, reg.edi, bios_interrupt)
    }
}
