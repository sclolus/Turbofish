use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;

use core::convert::TryInto;
use libc_binding::stat;

pub fn sys_stat(filename: *const c_char, buf: *mut stat) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let (_safe_filename, _safe_buf) = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            let c_string: CString = (&v, filename).try_into()?;

            (
                unsafe {
                    core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                        filename as *const u8,
                        c_string.len(),
                    ))
                },
                v.make_checked_ref_mut::<stat>(buf)?,
            )
        };
        // TODO: Open filename in VFS and get a Fd
        // TODO: Use VFS stat on FD (send stat structure)
        // TODO: Return
        Ok(0)
    })
}
