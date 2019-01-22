use crate::ffi::c_str;

#[derive(Clone, Copy, Debug)]
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

#[repr(C)]
struct Symbol {
    offset: u32,
    name: c_str,
}

#[no_mangle]
extern "C" {
    fn _init_backtrace(initial_ebp: u32) -> ();
    fn _get_eip() -> u32;
    fn _get_symbol(eip: u32) -> Symbol;
}

#[no_mangle]
pub extern "C" fn panic_handler(s: c_str, ext_reg: ExtendedRegisters) -> () {
    println!("KERNEL PANIC !");
    println!("reason {:?}", s);
    println!("{:#X?}\n", ext_reg);
    unsafe {
        _init_backtrace(ext_reg.new_ebp); // XXX put old_ebp in real ISR situation
    }

    loop {
        let eip;
        unsafe {
            eip = _get_eip();
        }
        if eip == 0 {
            break;
        }
        let symbol;
        unsafe {
            symbol = _get_symbol(eip);
        }
        println!("{:X?} : {:?}, eip={:X?}", symbol.offset, symbol.name, eip);
    }
    loop {}
}
