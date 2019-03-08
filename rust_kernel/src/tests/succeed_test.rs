//! This is an exemple of a basic test
//! you can `exit_qemu(0)` to pass the test
//! you can `exit_qemu(1)` or `panic!` to fail the test
//! you can `eprintln!` to print to the serial console after calling `UART_16550.init()`
use crate::io::UART_16550;
use crate::multiboot::MultibootInfo;
use crate::tests::helpers::exit_qemu;

#[no_mangle]
pub extern "C" fn kmain(_multiboot_info: *const MultibootInfo) -> u32 {
    unsafe {
        UART_16550.init();
    }
    eprintln!("THIS IS A BASIC TEST");
    exit_qemu(0);
    0
}
