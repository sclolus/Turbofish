use crate::ffi::c_str;

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct ExtendedRegisters {
    /*0       |*/ pub ds: u32,
    /*4       |*/ pub es: u32,
    /*8       |*/ pub fs: u32,
    /*12      |*/ pub gs: u32,
    /*16      |*/ pub ss: u32,
    /*20      |*/ pub eip: u32,
    /*24      |*/ pub cs: u32,
    /*28      |*/ pub eflags: u32,
    /*32      |*/ pub edi:u32,
    /*36      |*/ pub esi:u32,
    /*40      |*/ pub new_ebp:u32,
    /*44      |*/ pub esp:u32,
    /*48      |*/ pub ebx:u32,
    /*52      |*/ pub edx:u32,
    /*56      |*/ pub ecx:u32,
    /*60      |*/ pub eax:u32,
    /*64      |*/ pub old_ebp: u32,
    /*68      |*/
}

#[no_mangle]
pub extern "C" fn panic_handler(s: c_str, ext_reg: ExtendedRegisters) -> () {
    println!("KERNEL PANIC !");
    println!("reason {:?}", s);
    println!("{:#X?}\n", ext_reg);
    loop {}
}
