#![cfg_attr(not(test), no_std)]

#[no_mangle]
pub extern "C" fn rustmain() -> i32 {
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

static STRING: &str = "I never used GNU/LINUX distribution.\n";

extern "C" {
    fn user_write(fd: i32, s: *const u8, len: usize) -> i32;
}
