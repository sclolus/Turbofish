//! The safe FFI si an extend of the basic FFI with Safe conversion methods from C types to Rust Types

pub use crate::ffi::{c_char, CString, CStringArray};
use crate::memory::{tools::PAGE_SIZE, AddressSpace};

use fallible_collections::{try_vec, FallibleVec};
use libc_binding::Errno;
use sync::DeadMutexGuard;

use alloc::vec::Vec;

use core::convert::TryInto;

/// Get the len of a C style *const c_char. Operate in a limited area
fn safe_strlen(ptr: *const c_char, limit: usize) -> Option<usize> {
    let mut i = 0;
    while i != limit && unsafe { (*ptr.add(i)).0 } != 0 {
        i += 1;
    }
    if i == limit {
        None
    } else {
        Some(i)
    }
}

/// Secure create a CString from a C char*
impl core::convert::TryFrom<(&DeadMutexGuard<'_, AddressSpace>, *const c_char)> for CString {
    type Error = Errno;
    fn try_from(
        arg: (&DeadMutexGuard<'_, AddressSpace>, *const c_char),
    ) -> Result<Self, Self::Error> {
        let mut string_len = 0;

        let mut curr_ptr = arg.1;
        loop {
            // Check if the pointer exists in address space
            arg.0.check_user_ptr::<c_char>(curr_ptr)?;
            // Set the remaining length, relative to page size
            let limit = PAGE_SIZE - (curr_ptr as usize) % PAGE_SIZE;
            let res = safe_strlen(curr_ptr, limit);
            if let Some(len) = res {
                // In case of success, set the final string_len and break
                string_len += len;
                break;
            } else {
                // In case of failure, advance string_len & curr_ptr
                string_len += limit;
                curr_ptr = (curr_ptr as usize + limit) as _;
            }
        }

        let v: Vec<c_char> = unsafe { core::mem::transmute(try_vec![0 as u8; string_len + 1]?) };
        unsafe {
            (v.as_ptr() as *mut u8).copy_from(arg.1 as _, string_len);
        }
        Ok(Self(v))
    }
}

/// Secure create a CStringArray from a C char** type
impl core::convert::TryFrom<(&DeadMutexGuard<'_, AddressSpace>, *const *const c_char)>
    for CStringArray
{
    type Error = Errno;
    fn try_from(
        arg: (&DeadMutexGuard<'_, AddressSpace>, *const *const c_char),
    ) -> Result<Self, Self::Error> {
        // tips: Constructs a new, empty Vec<T>. The vector will not allocate until elements are pushed onto it.
        let mut c_pointer: Vec<*const c_char> = Vec::new();
        let mut borrowed_content: Vec<CString> = Vec::new();

        // Direct NULL pointer case
        if arg.1 != 0x0 as _ {
            unsafe {
                let mut curr_ptr = arg.1;
                let pointer_size = core::mem::size_of::<*const char>();
                loop {
                    // Check if the pointer exists in address space
                    arg.0.check_user_ptr::<*const c_char>(curr_ptr)?;
                    // Set the remaining length, relative to page size
                    let limit = PAGE_SIZE - (curr_ptr as usize) % PAGE_SIZE;

                    let mut i = 0;
                    while i != limit && *(curr_ptr.add(i / pointer_size)) != core::ptr::null() {
                        let string: CString =
                            (arg.0, (*(curr_ptr.add(i / pointer_size)) as _)).try_into()?;
                        c_pointer.try_push(string.as_ptr())?;
                        borrowed_content.try_push(string)?;
                        i += pointer_size;
                    }
                    if i != limit {
                        break;
                    }
                    curr_ptr = (curr_ptr as usize + limit) as _;
                }
            }
        }

        // nullptr to terminate the array
        c_pointer.try_push(0x0 as _)?;
        Ok(Self {
            c_pointer,
            borrowed_content,
        })
    }
}
