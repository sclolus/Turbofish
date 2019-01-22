use core::fmt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct c_char(pub u8);

impl fmt::Debug for c_char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

pub unsafe extern "C" fn strlen(ptr: *const c_char) -> usize {
    let mut i = 0;
    while (*ptr.offset(i as isize)).0 != 0 {
        i += 1;
    }
    i
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct c_str {
    pub ptr: *const c_char,
}

impl fmt::Debug for c_str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(self.ptr as *const u8, strlen(self.ptr)); // Make slice of u8 (&[u8])
            write!(f, "{}", core::str::from_utf8_unchecked(slice)) // Make str slice (&[str]) with &[u8]
        }
    }
}

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
macro_rules! define_raw_data {
    ($name:ident, $size_in_bytes:expr) => {
        #[derive(Copy, Clone)]
        #[repr(C)]
        #[repr(packed)]
        pub struct $name(pub [u8; $size_in_bytes]);
        impl_raw_data_debug!($name);
    };
}
