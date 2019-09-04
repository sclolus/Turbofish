//! The safe FFI si an extend of the basic FFI with Safe conversion methods from C types to Rust Types

pub use crate::ffi::{c_char, CString, CStringArray};
use crate::memory::{tools::PAGE_SIZE, AddressSpace};

use fallible_collections::{try_vec, FallibleVec};
use libc_binding::Errno;
use sync::DeadMutexGuard;

use alloc::vec::Vec;

use core::convert::TryInto;

/// Secure create a CString from a C char*
impl core::convert::TryFrom<(&DeadMutexGuard<'_, AddressSpace>, *const c_char)> for CString {
    type Error = Errno;
    fn try_from(
        arg: (&DeadMutexGuard<'_, AddressSpace>, *const c_char),
    ) -> Result<Self, Self::Error> {
        let s = arg
            .0
            .make_checked_str(arg.1 as *const libc_binding::c_char)?;
        let v: Vec<c_char> = unsafe { core::mem::transmute(try_vec![0 as u8; s.len() + 1]?) };

        unsafe {
            (v.as_ptr() as *mut u8).copy_from(s.as_ptr() as _, s.len());
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
