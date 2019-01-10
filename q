[1mdiff --git a/nucleus/src/lib.rs b/nucleus/src/lib.rs[m
[1mindex 8dba492..0d6052d 100644[m
[1m--- a/nucleus/src/lib.rs[m
[1m+++ b/nucleus/src/lib.rs[m
[36m@@ -4,12 +4,55 @@[m
 #![no_std][m
 #![feature(compiler_builtins_lib)][m
 [m
[31m-//extern crate rlibc;[m
[31m-[m
 pub mod support; // For Rust lang items[m
[32m+[m[32muse core::fmt::Arguments;[m
[32m+[m[32muse core::fmt::write;[m
[32m+[m[32muse core::fmt::Write;[m
[32m+[m[32muse core::fmt::Result;[m
 [m
 #[no_mangle][m
 pub extern "C" fn kmain() {[m
[32m+[m[32m    let mut pos = putchar_vga(1, 'H', 12);[m
[32m+[m[32m    pos = putchar_vga(pos, 'E', 13);[m
[32m+[m[32m    pos = putchar_vga(pos, 'L', 14);[m
[32m+[m[32m    pos = putchar_vga(pos, 'L', 15);[m
[32m+[m[32m    pos = putchar_vga(pos, 'O', 16);[m
[32m+[m[32m    pos = putchar_vga(pos, ' ', 17);[m
[32m+[m[32m    pos = putchar_vga(pos, 'W', 18);[m
[32m+[m[32m    pos = putchar_vga(pos, 'O', 19);[m
[32m+[m[32m    pos = putchar_vga(pos, 'R', 20);[m
[32m+[m[32m    pos = putchar_vga(pos, 'L', 21);[m
[32m+[m[32m    pos = putchar_vga(pos, 'D', 22);[m
[32m+[m[32m    pos = putchar_vga(pos, '!', 23);[m
[32m+[m[32m    pos = putstring_vga(pos, "LALA", 8);[m
[32m+[m[32m    let mut vga = Vga {};[m
[32m+[m[32m    write(&mut vga, format_args!("hello {:?}", 12));[m
     loop { }[m
 }[m
 [m
[32m+[m[32mstruct Vga {[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mimpl Write for Vga {[m
[32m+[m[32m    fn write_str(&mut self, s: &str) -> Result {[m
[32m+[m[32m        putstring_vga(140, s, 8);[m
[32m+[m[32m        Ok(())[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mfn putstring_vga(mut pos: isize, s:&str, color: u8) -> isize {[m
[32m+[m[32m    for c in s.as_bytes() {[m
[32m+[m[32m        pos = putchar_vga(pos, *c as char, color);[m
[32m+[m[32m    }[m
[32m+[m[32m    pos[m
[32m+[m[32m}[m
[32m+[m
[32m+[m[32mfn putchar_vga(pos:isize, c:char, color:u8) -> isize {[m
[32m+[m[32m    let ptr = 0xB8000 as *mut u8;[m
[32m+[m
[32m+[m[32m    unsafe {[m
[32m+[m[32m        *ptr.offset(pos * 2) = c as u8;[m
[32m+[m[32m        *ptr.offset(pos * 2 + 1) = color;[m
[32m+[m[32m    }[m
[32m+[m[32m    pos + 1[m
[32m+[m[32m}[m
