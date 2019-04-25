#![no_std]
/// implement debug for tuple struct slice
#[macro_export(local_inner_macros)]
macro_rules! impl_raw_data {
    ($e:ty, $size_in_bytes:expr) => {
        impl core::fmt::Debug for $e {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::write!(f, "raw data at address {:?}: {} bytes reserved", &self.0 as *const u8, self.0.len())
            }
        }
        impl core::cmp::PartialEq for $e {
            fn eq(&self, other: &Self) -> bool {
                self.0[..] == other.0[..]
            }
        }
        impl core::default::Default for $e {
            fn default() -> Self {
                Self([0; $size_in_bytes])
            }
        }
    };
}

/// create a tuple struct of type $name containing a slice wich implements debug
/// useful because rust does't implement debug for slice > 32 elem
#[macro_export]
macro_rules! define_raw_data {
    ($name:ident, $size_in_bytes:expr) => {
        #[derive(Copy, Clone)]
        #[repr(C)]
        #[repr(packed)]
        pub struct $name(pub [u8; $size_in_bytes]);
        $crate::impl_raw_data!($name, $size_in_bytes);
    };
}
