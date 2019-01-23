pub const POISON_SLAB: u32 = 0x5a5a5a5a;

// Returns a &[str] containing the full namespace specified name of the function

// This works by declaring a dummy function f() nested in the current function.
// Then by the type_name instrinsics, get the slice of the full specified name of the function f()
// we then truncate the slice by the range notation to the name of the current function.
// That is the slice with 5 characters removed.
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
