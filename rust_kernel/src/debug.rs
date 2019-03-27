use crate::drivers::pit_8253::*;

pub const POISON_SLAB: u32 = 0x5a5a5a5a;

///Returns a &str containing the full namespace specified name of the function
// This works by declaring a dummy function f() nested in the current function.
// Then by the type_name instrinsics, get the slice of the full specified name of the function f()
// we then truncate the slice by the range notation to the name of the current function.
// That is the slice with 5 characters removed.
#[allow(unused_macros)]
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }
        let name = type_name_of(f);
        &name[6..name.len() - 4]
    }};
}

/// copy of std dbg! macro
#[allow(unused_macros)]
#[macro_export]
macro_rules! dbg {
    ($val: expr) => {
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
}

extern "C" {
    fn _get_pic_time() -> u32;
}

static mut BENCH_START_TIME: u32 = 0;

/// start benchmark
/// # Warning
/// - the Interupts must be enabled
/// - the Pit irq must be enabled on the Pic
/// - the Pit must be initialized
pub fn bench_start() {
    unsafe {
        BENCH_START_TIME = _get_pic_time();
    }
}

/// return time elapsed is ms since bench_start in ms using the pit actual configuration
pub fn bench_end() -> u32 {
    unsafe { ((_get_pic_time() - BENCH_START_TIME) as f32 * PIT0.lock().period * 1000 as f32) as u32 }
}
