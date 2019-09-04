use super::SysResult;

use super::scheduler::SCHEDULER;
use libc_binding::{c_char, Errno};

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
        if size == 0 {
            return Err(Errno::EINVAL);
        }
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let safe_buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            v.make_checked_mut_slice(buf, size)?
        };

        let cwd = &scheduler.current_thread_group().cwd;
        let mut i = 0;
        for b in cwd.iter_bytes() {
            // keep a place for the \0
            if i >= size - 1 {
                return Err(Errno::ERANGE);
            }
            safe_buf[i] = *b;
            i += 1;
        }
        if i == 0 {
            panic!("cwd is empty");
        }
        safe_buf[i] = '\0' as c_char;
        Ok(0)
    })
}
