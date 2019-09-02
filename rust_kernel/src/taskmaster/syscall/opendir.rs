use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;

use core::convert::TryFrom;
use core::convert::TryInto;
use libc_binding::{dirent, DIR};

use crate::memory::tools::AllocFlags;

/// Return directory content in userspace memory
pub fn sys_opendir(filename: *const c_char, dir: *mut DIR) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let (safe_filename, safe_dir) = {
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
                v.make_checked_ref_mut::<DIR>(dir)?,
            )
        };

        let mut current = super::vfs::Current {
            cwd: super::vfs::DirectoryEntryId::new(2),
            uid: 0,
            euid: 0,
            gid: 0,
            egid: 0,
        };
        let path = super::vfs::Path::try_from(safe_filename)?;
        let dirent_vector = super::vfs::VFS.lock().opendir(&mut current, path)?;

        let size = dirent_vector.len() * core::mem::size_of::<dirent>();

        // Allocate a chunk of memory in userspace to store the dirent array
        let mut v = scheduler
            .current_thread()
            .unwrap_process()
            .get_virtual_allocator();
        let user_mem: *mut dirent = v.alloc(size, AllocFlags::USER_MEMORY)? as _;

        // Copy the dirent array from kernel_space and set the user 'DIR' structure
        unsafe {
            core::ptr::copy(dirent_vector.as_ptr(), user_mem, dirent_vector.len());
        }
        safe_dir.current_offset = 0;
        safe_dir.length = dirent_vector.len();
        safe_dir.array = user_mem;
        Ok(0)
    })
}
