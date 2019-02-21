/// This file contains the macros for making constant assertions that fails at compile time.

#[doc(hidden)]
#[cfg(not(feature = "nightly"))]
#[macro_export(local_inner_macros)]
macro_rules! _const_assert {
    ($($xs:expr),+ $(,)*) => {
        const _ : [(); 0 - !($($xs)&&+) as usize] = [];
    };
    // ($label:ident; $($xs:tt)+) => {
    //     #[allow(dead_code, non_snake_case)]
    //     fn $label() { const_assert!($($xs)+); }
    // };
}

#[doc(hidden)]
#[cfg(feature = "nightly")]
#[macro_export(local_inner_macros)]
#[allow(dead_code)]
#[allow(unknown_lints, eq_op)]
macro_rules! _const_assert {
    ($($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !($($xs)&&+) as usize] = [];
    };
}

/// This macro enable us to make assertions that are evaluated at compile time, if a constant assertion is false, then the compilation fails.
/// This macro can be used with any constant expressions, fonctions can be used if they are const qualified.
/// This macro is directly taken from the static_assertions crate.
#[macro_export(local_inner_macros)]
macro_rules! const_assert {
    ($($xs:tt)+) => { _const_assert!($($xs)+); };
}
