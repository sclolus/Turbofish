/// This file contains the macros for making constant assertions that fails at compile time.

/// This macro enable us to make assertions that are evaluated at compile time, if a constant assertion is false, then the compilation fails.
/// This macro can be used with any constant expressions, fonctions can be used if they are const qualified.
/// This macro is directly taken from the static_assertions crate.
#[allow(unused_macros)]
macro_rules! const_assert {
    ($($xs:expr),+ $(,)*) => {
        #[allow(unknown_lints, eq_op)]
        const _ : [(); 0 - !($($xs)&&+) as usize] = [];
    };
}

#[cfg(test)]
mod test {

    #[allow(dead_code)]
    struct Example {
        _1: usize,
        _2: usize,
    }

    #[test]
    fn test_const_assert() {
        const_assert!(true, true, 1 + 1 == 2);
        const_assert!(true,,,);
        #[cfg(not(test))]
        const_assert!(core::mem::size_of::<Example>() == 8);
        #[cfg(not(test))]
        const_assert!(core::mem::size_of::<usize>() == 4);
    }
}
