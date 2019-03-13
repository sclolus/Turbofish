use crate::ffi::c_str;

#[derive(Debug, Copy, Clone)]
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

extern "C" {
    fn _get_symbol(eip: u32) -> Symbol;
}

#[repr(C)]
struct Symbol {
    offset: u32,
    name: c_str,
}

/// Get eip from ebp
// return tupple of (eip, ebp)
fn get_eip(ebp: *const u32) -> (u32, *const u32) {
    let eip = unsafe { *ebp.add(1) };
    if eip == 0 {
        (0, ebp)
    } else {
        (eip, unsafe { *ebp as *const u32 })
    }
}

fn trace_back(ebp_origin: *const u32) {
    let mut s: (u32, *const u32) = (0, ebp_origin);
    loop {
        s = get_eip(s.1);
        if s.0 == 0 {
            break;
        }
        let symbol = unsafe { _get_symbol(s.0) };
        eprintln!("{:X?} : {:?}, eip={:X?}", symbol.offset, symbol.name, s.0);
    }
}

#[no_mangle]
pub extern "C" fn cpu_panic_handler(s: c_str, ext_reg: ExtendedRegisters) -> () {
    println!("KERNEL PANIC !");
    println!("reason {:?}", s);
    println!("{:#X?}\n", ext_reg);

    trace_back(ext_reg.new_ebp as *const u32);
    loop {}
}

use core::panic::PanicInfo;

#[allow(dead_code)]
fn panic_sa_mere(info: &PanicInfo) {
    eprintln!("Rust is on panic but it is not a segmentation fault !\n{}", info);
    let ebp: *const u32;
    unsafe { asm!("mov eax, ebp" : "={eax}"(ebp) : : : "intel") }
    trace_back(ebp);
}

#[panic_handler]
#[cfg(not(feature = "exit-on-panic"))]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    panic_sa_mere(info);
    loop {}
}

#[panic_handler]
#[cfg(feature = "exit-on-panic")] //for integration test when not in graphical
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    panic_sa_mere(info);
    use crate::tests::helpers::exit_qemu;
    exit_qemu(1);
    loop {}
}
