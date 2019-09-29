use core::fmt;

/// Get the len of a C style *const c_char
pub unsafe fn strlen(ptr: *const c_char) -> usize {
    let mut i = 0;
    while (*ptr.add(i)).0 != 0 {
        i += 1;
    }
    i
}

/// Main structure of c_char
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct c_char(pub u8);

/// Main structure of c_str
#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct c_str {
    pub ptr: *const c_char,
}

/// Debug boilerplate of c_char
impl fmt::Debug for c_char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

/// Debug boilerplate of c_str
impl fmt::Debug for c_str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            // Make slice of u8 (&[u8])
            let slice: &[u8] = core::slice::from_raw_parts(self.ptr as *const u8, strlen(self.ptr));
            // Make str slice (&[str]) with &[u8]
            write!(
                f,
                "{}",
                core::str::from_utf8(slice).expect("c_str is not utf8")
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ffi::{c_char, strlen};
    #[test]
    fn test_strlen() {
        unsafe {
            let s = "12345\0";
            assert_eq!(strlen(s as *const _ as *const c_char), 5);
            let s = "\0";
            assert_eq!(strlen(s as *const _ as *const c_char), 0);
        }
    }
}
