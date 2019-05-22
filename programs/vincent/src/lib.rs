#![cfg_attr(not(test), no_std)]

#[no_mangle]
extern "C" fn _start() -> ! {
    let ret = main();
    unsafe {
        user_exit(ret)
    }
}

fn main() -> i32 {
    unsafe {
        user_write(1, STRING.as_ptr(), STRING.len());
    }
    0
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static STRING: &str = "I made 42sh.\n";

extern "C" {
    fn user_write(fd: i32, s: *const u8, len: usize) -> i32;
    fn user_exit(return_value: i32) -> !;
}
