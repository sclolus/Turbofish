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

/*
In a call scheme like that:

fn main(void) -> fn A(int i) -> fn B (int i, int j) -> fn C
when you took current EBP value in function named 'C',
EIP of caller (here fn B) is just after !

CALL Anatomy of function 'C' by function 'B':
---------------------------------------------
...
push i
PUSH EIP
Go to fn B
push ebp
mov ebp, esp
...            | Some code in B fonction
push j         | CALL second argument
push i         | CALL first argument
push EIP       | CALL code offset when function 'C' is called
Go to fn C     | CALL
push ebp       | begin of function C ; EBP of caller
mov ebp, esp   | begin of function C ; EBP = ESP so EBP[1] = EIP of caller
...            | Some code in C function

<------------------------------------------------------------------------|
Stack anatomy
 ... | EBP | EIP | arg1 | arg2 | ... | EBP | EIP | arg1 | ...
   stack frame C |        stack frame B          |  stack frame A
<------------------------------------------------------------------------|
set EBP variable as pointer -> u32 *EBP = (U32 *)current_ebp
to find EIP get EBP[1] -> EIP = EBP[1] (EIP is in stack frame B)
reset EBP -> EBP = *EBP (Go to EBP location in stack frame B)
to find EIP get EBP[1] -> EIP = EBP[1] (EIP is in stack frame A)
reset EBP -> EBP = *EBP (Go to EBP location in stack frame A)
*/

static mut EBP: *const u32 = 0x0 as *const u32;

/// Get eip from global variable EBP
unsafe fn get_eip() -> u32 {
    let eip: u32 = *EBP.add(1);
    if eip == 0 {
        return 0;
    }
    EBP = *EBP as *const u32;
    eip
}

#[repr(C)]
struct Symbol {
    offset: u32,
    name: c_str,
}

extern "C" {
    fn _get_symbol(eip: u32) -> Symbol;
}

#[no_mangle]
pub extern "C" fn panic_handler(s: c_str, ext_reg: ExtendedRegisters) -> () {
    println!("KERNEL PANIC !");
    println!("reason {:?}", s);
    println!("{:#X?}\n", ext_reg);
    unsafe {
        // TODO put old_ebp in real ISR situation
        EBP = ext_reg.new_ebp as *const u32;
    }
    loop {
        let eip;
        unsafe {
            eip = get_eip();
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
