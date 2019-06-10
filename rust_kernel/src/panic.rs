use crate::ffi::c_str;
use crate::system::ExtendedRegisters;
use crate::interrupts;
use core::panic::PanicInfo;

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

/// Take the first eip and epb as parameter and trace back up.
pub fn trace_back(mut s: (u32, *const u32)) {
    loop {
        let symbol = unsafe { _get_symbol(s.0) };
        eprintln!("{:X?} : {:?}, eip={:X?}", symbol.offset, symbol.name, s.0);
        s = get_eip(s.1);
        if s.0 == 0 {
            break;
        }
    }
}

extern "C" {
    fn _read_cr2() -> u32;
}

use crate::memory::KERNEL_VIRTUAL_PAGE_ALLOCATOR;

#[no_mangle]
pub extern "C" fn cpu_page_fault_handler(cr2: u32, err_code: u32, ext_reg: ExtendedRegisters) -> () {
    let virtual_page_allocator = unsafe { KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap() };
    if let Err(e) = virtual_page_allocator.valloc_handle_page_fault(cr2) {
        use bit_field::BitField;
        let page_fault_cause = match err_code.get_bits(0..3) {
            0b000 => "Supervisory process tried to read a non-present page entry",
            0b001 => "Supervisory process tried to read a page and caused a protection fault",
            0b010 => "Supervisory process tried to write to a non-present page entry",
            0b011 => "Supervisory process tried to write a page and caused a protection fault",
            0b100 => "User process tried to read a non-present page entry",
            0b101 => "User process tried to read a page and caused a protection fault",
            0b110 => "User process tried to write to a non-present page entry",
            0b111 => "User process tried to write a page and caused a protection fault",
            _ => "WTF",
        };
        eprintln!("err_code: {:X}", err_code);
        eprintln!("{}", page_fault_cause);
        eprintln!("cr2: 0x{:x}", cr2);
        eprintln!("{:?}", e);
        eprintln!("{:X?}\n", ext_reg);

        if ext_reg.cs == 0x08 {
            trace_back((ext_reg.eip, ext_reg.old_ebp as *const u32));
        } else {
            eprintln!("Cannot display backtrace from a non-kernel routine !");
        }
        qemu_check();
        loop {}
    };
}

#[no_mangle]
pub extern "C" fn cpu_panic_handler(s: c_str, ext_reg: ExtendedRegisters) -> () {
    unsafe {
        interrupts::disable();
    }
    eprintln!("KERNEL PANIC !\nreason {:?}\n{:X?}", s, ext_reg);

    if ext_reg.cs == 0x08 {
        trace_back((ext_reg.eip, ext_reg.old_ebp as *const u32));
    } else {
        eprintln!("Cannot display backtrace from a non-kernel routine !");
    }
    qemu_check();
    loop {}
}

#[allow(dead_code)]
pub fn panic_sa_mere(info: &PanicInfo) {
    unsafe {
        interrupts::disable();
    }
    eprintln!("Rust is on panic but it is not a segmentation fault !\n{}", info);
    let ebp: *const u32;
    unsafe {
        asm!("mov eax, ebp" : "={eax}"(ebp) : : : "intel");
        trace_back((*ebp.add(1), *ebp as *const u32));
    };
}

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    panic_sa_mere(info);
    qemu_check();
    loop {}
}

fn qemu_check() {
    #[cfg(feature = "exit-on-panic")]
    {
        // for integration test
        use crate::tests::helpers::exit_qemu;
        exit_qemu(1);
    }
}
