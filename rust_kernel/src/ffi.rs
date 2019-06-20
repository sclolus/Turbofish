use alloc::vec;
use alloc::vec::Vec;

use core::convert;
use core::fmt;

/// Main structure of c_char
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct c_char(u8);

/// Main structure of c_str
#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct c_str {
    pub ptr: *const c_char,
}

/// Main structure of CString
pub struct CString(Vec<c_char>);

/// Main structure of CStringArray
pub struct CStringArray {
    /// Pointer vector of C Style
    c_pointer: Vec<*const c_char>,
    /// Rust borrowed content
    borrowed_content: Vec<CString>,
}

/// Debug boilerplate of c_char
impl fmt::Debug for c_char {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

/// Get the len of a C style *const c_char
pub unsafe fn strlen(ptr: *const c_char) -> usize {
    let mut i = 0;
    while (*ptr.add(i)).0 != 0 {
        i += 1;
    }
    i
}

/// Debug boilerplate of c_str
impl fmt::Debug for c_str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            // Make slice of u8 (&[u8])
            let slice: &[u8] = core::slice::from_raw_parts(self.ptr as *const u8, strlen(self.ptr));
            // Make str slice (&[str]) with &[u8]
            write!(f, "{}", core::str::from_utf8_unchecked(slice))
        }
    }
}

impl CString {
    /// Return a C char* pointer from a CString
    pub fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const c_char
    }

    /// Return the string length
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }
}

/// Create a CString from a RUST str slice
impl convert::From<&str> for CString {
    fn from(rust_str: &str) -> Self {
        let v: Vec<c_char> = vec![c_char(0); rust_str.len() + 1];
        unsafe {
            (v.as_ptr() as *mut u8).copy_from(rust_str.as_ptr(), rust_str.len());
        }
        Self(v)
    }
}

/// Create a CString from a C char*
impl convert::From<*const c_char> for CString {
    fn from(c_str: *const c_char) -> Self {
        let len = unsafe { strlen(c_str) };
        let v: Vec<c_char> = vec![c_char(0); len + 1];
        unsafe {
            (v.as_ptr() as *mut u8).copy_from(c_str as *const u8, len);
        }
        Self(v)
    }
}

/// Debug boilerplate of CString
impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug_slice = unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.len()) };
        write!(f, "{}", unsafe { core::str::from_utf8_unchecked(debug_slice) })
    }
}

impl CStringArray {
    /// Return a C char** from a RUST CStringArray
    pub fn as_ptr(&self) -> *const *const c_char {
        self.c_pointer.as_ptr()
    }
}

/// Create a CStringArray from a RUST str slice array
impl convert::From<&[&str]> for CStringArray {
    fn from(rust_str_array: &[&str]) -> Self {
        let mut c_pointer: Vec<*const c_char> = Vec::new();
        let mut borrowed_content: Vec<CString> = Vec::new();

        for &elem in rust_str_array.iter() {
            let string: CString = elem.into();
            c_pointer.push(string.as_ptr());
            borrowed_content.push(string);
        }
        // nullptr to terminate the array
        c_pointer.push(0x0 as *const c_char);
        Self { c_pointer, borrowed_content }
    }
}

/// Create a CStringArray from a C char** type
impl convert::From<*const *const c_char> for CStringArray {
    fn from(sa: *const *const c_char) -> Self {
        let mut c_pointer: Vec<*const c_char> = Vec::new();
        let mut borrowed_content: Vec<CString> = Vec::new();

        let mut i = 0;
        unsafe {
            while *(sa.add(i)) != 0x0 as *const c_char {
                let string: CString = (*(sa.add(i)) as *const c_char).into();
                c_pointer.push(string.as_ptr());
                borrowed_content.push(string);
                i += 1;
            }
        }
        // nullptr to terminate the array
        c_pointer.push(0x0 as *const c_char);
        Self { c_pointer, borrowed_content }
    }
}

/// Debug boilerplate of CStringArray
impl fmt::Debug for CStringArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.borrowed_content {
            write!(f, "{:?}\n", elem)?;
        }
        Ok(())
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
