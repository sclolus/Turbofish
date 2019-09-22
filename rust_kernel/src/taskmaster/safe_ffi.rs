//! The safe FFI si an extend of the basic FFI with Safe conversion methods from C types to Rust Types

use libc_binding::c_char;

use alloc::collections::CollectionAllocErr;
use alloc::vec::Vec;

use core::convert;
use core::convert::TryInto;
use core::fmt;
use fallible_collections::{try_vec, FallibleVec, TryClone};

/// Main structure of CString
pub struct CString(pub Vec<c_char>);

/// Main structure of CStringArray
#[derive(Debug)]
pub struct CStringArray {
    /// Pointer vector of C Style
    pub c_pointer: Vec<*const c_char>,
    /// Rust borrowed content
    pub owned_content: Vec<CString>,
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

    /// Get the len of data if the are serialized into a raw buffer
    pub fn get_serialized_len(&self, align: usize) -> Option<usize> {
        if align == 0 {
            None
        } else if self.0.len() % align == 0 {
            Some(self.0.len())
        } else {
            Some(self.0.len() + align - self.0.len() % align)
        }
    }

    /// Serialize the data into a raw buffer beginning at ptr location
    pub fn serialize(&self, align: usize, ptr: *mut c_char) -> Option<*mut c_char> {
        if align == 0 {
            None
        } else {
            let aligned_ptr = (ptr as usize - ptr as usize % align) as *mut c_char;
            unsafe {
                aligned_ptr.copy_from(self.as_ptr(), self.0.len());
            }
            Some(aligned_ptr)
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &i8> {
        self.0.iter()
    }
}

/// Create a CString from a RUST str slice
impl convert::TryFrom<&str> for CString {
    type Error = CollectionAllocErr;
    fn try_from(rust_str: &str) -> Result<Self, Self::Error> {
        let v: Vec<c_char> = try_vec![0; rust_str.len() + 1]?;
        unsafe {
            (v.as_ptr() as *mut u8).copy_from(rust_str.as_ptr(), rust_str.len());
        }
        Ok(Self(v))
    }
}

/// Debug boilerplate of CString
impl fmt::Debug for CString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug_slice =
            unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.len()) };
        write!(
            f,
            "{} of len: {}",
            unsafe { core::str::from_utf8_unchecked(debug_slice) },
            self.len()
        )
    }
}

/// Display boilerplate of CString
impl fmt::Display for CString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let debug_slice =
            unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.len()) };
        write!(f, "{}", unsafe {
            core::str::from_utf8_unchecked(debug_slice)
        })
    }
}

impl CStringArray {
    pub fn strings(&self) -> impl Iterator<Item = &CString> {
        self.owned_content.iter()
    }

    /// Return a C char** from a RUST CStringArray
    pub fn as_ptr(&self) -> *const *const c_char {
        self.c_pointer.as_ptr()
    }

    /// Get the number of contained strings
    pub fn len(&self) -> usize {
        self.owned_content.len()
    }

    /// Get the len of data if the are serialized into a raw buffer
    pub fn get_serialized_len(&self, align: usize) -> Option<usize> {
        if align == 0 {
            None
        } else {
            // First, count the string pointers total len
            let mut total_len = self.c_pointer.len() * core::mem::size_of::<*const c_char>();

            // Fix unaligned
            if total_len % align != 0 {
                total_len += align - total_len % align
            }

            // Then, add all the strings len
            for elem in self.owned_content.iter() {
                total_len += elem.get_serialized_len(align).expect("WTF");
            }
            Some(total_len)
        }
    }

    /// Serialize the data into a raw buffer beginning at ptr location
    pub fn serialize(&self, align: usize, ptr: *mut c_char) -> Option<*mut *mut c_char> {
        if align == 0 {
            None
        } else {
            // Align the pointer
            let origin_aligned_ptr = (ptr as usize - ptr as usize % align) as *mut *mut c_char;
            let mut aligned_ptr = origin_aligned_ptr as usize;

            // First, reserve space for the string pointers
            let len = self.c_pointer.len() * core::mem::size_of::<*mut c_char>();
            aligned_ptr += len;
            // Fix unaligned
            if aligned_ptr % align != 0 {
                aligned_ptr += align - aligned_ptr % align;
            }

            // Then, copy all the strings
            for (i, elem) in self.owned_content.iter().enumerate() {
                let res = elem
                    .serialize(align, aligned_ptr as *mut c_char)
                    .expect("WTF");
                // check align coherency
                if res as usize != aligned_ptr {
                    return None;
                }
                unsafe {
                    // Make one BSP entry
                    *(origin_aligned_ptr.add(i)) = aligned_ptr as *mut c_char;
                }
                aligned_ptr += elem.get_serialized_len(align).expect("WTF");
            }
            unsafe {
                // Terminate the array of string pointer by a nulltpr
                *(origin_aligned_ptr.add(self.c_pointer.len() - 1)) = 0x0 as *mut c_char;
            }
            Some(origin_aligned_ptr)
        }
    }
}

impl TryClone for CString {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        let inner = self.0.try_clone()?;

        Ok(Self(inner))
    }
}

impl TryClone for CStringArray {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        let mut owned_content = Vec::try_with_capacity(self.len())?;
        let mut c_pointer = Vec::try_with_capacity(self.len() + 1)?;

        for string in self.owned_content.iter() {
            let copied = string.try_clone()?;
            c_pointer.try_push(copied.as_ptr())?;
            owned_content.try_push(copied)?;
        }
        Ok(Self {
            c_pointer,
            owned_content,
        })
    }
}

/// Create a CStringArray from a RUST str slice array
impl convert::TryFrom<&[&str]> for CStringArray {
    type Error = CollectionAllocErr;
    fn try_from(rust_str_array: &[&str]) -> Result<Self, Self::Error> {
        let mut c_pointer: Vec<*const c_char> = Vec::new();
        let mut owned_content: Vec<CString> = Vec::new();

        for &elem in rust_str_array.iter() {
            let string: CString = elem.try_into()?;
            c_pointer.try_push(string.as_ptr())?;
            owned_content.push(string);
        }
        // nullptr to terminate the array
        c_pointer.try_push(0x0 as *const c_char)?;
        Ok(Self {
            c_pointer,
            owned_content,
        })
    }
}

/// Debug boilerplate of CStringArray
impl fmt::Display for CStringArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.owned_content {
            write!(f, "{:?}\n", elem)?;
        }
        Ok(())
    }
}
