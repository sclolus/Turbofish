
#[repr(C)]
struct Mastruct {
    lala: u32,
    lolo: u32,
}

extern {
    fn c_jump(a: u32, ptr: *const  u32, mastruc: Mastruct) -> u32;
}

#[no_mangle]
extern "C" fn rust_jump(_val: u32) -> u32 {
    let a = 2;
    let mastruc = Mastruct {lala:3, lolo:4};
    unsafe {
        c_jump(1, &a, mastruc);
    }
    return 42;
}
