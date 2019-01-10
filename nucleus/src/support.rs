#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}
/*#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

#[lang = "begin_unwind"]
pub extern "C" fn begin_unwind() {
}*/

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8,
                                src: *const u8,
                                n: usize)
                                -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i16, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

use crate::Vga;
pub static mut vga: Vga = Vga {width: 80, height: 25, x: 1, y: 1, color: 3};

#[macro_export]
//#[stable(feature = "rust1", since = "1.0.0")]
//#[allow_internal_unstable]
macro_rules! println {
    () => (print!("\n"));
($($arg:tt)*) => ({
    unsafe {
        write(&mut support::vga, format_args!($($arg)*)).unwrap();
    }
})
}
