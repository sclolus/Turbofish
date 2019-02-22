/// implement debug for tuple struct slice
macro_rules! impl_raw_data_debug {
    ($e:ty) => {
        impl core::fmt::Debug for $e {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "raw data at address {:?}: {} bytes reserved", &self.0 as *const u8, self.0.len())
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
        impl_raw_data_debug!($name);
    };
}
