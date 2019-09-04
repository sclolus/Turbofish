use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;

use core::convert::TryFrom;
use core::convert::TryInto;
use libc_binding::{dirent, DIR};

use crate::memory::tools::AllocFlags;

/// The getcwd() function shall place an absolute pathname of the
/// current working directory in the array pointed to by buf, and
/// return buf. The pathname shall contain no components that are dot
/// or dot-dot, or are symbolic links.
///
/// If there are multiple pathnames that getcwd() could place in the
/// array pointed to by buf, one beginning with a single <slash>
/// character and one or more beginning with two <slash> characters,
/// then getcwd() shall place the pathname beginning with a single
/// <slash> character in the array. The pathname shall not contain any
/// unnecessary <slash> characters after the leading one or two
/// <slash> characters.
///
/// The size argument is the size in bytes of the character array
/// pointed to by the buf argument. If buf is a null pointer, the
/// behavior of getcwd() is unspecified.
pub fn sys_getcwd(buf: *mut c_char, size: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_mut_slice(buf, size)?
        };

        //TODO: fill buf with cwd
        Ok(0)
    })
}
