use super::SysResult;

use super::safe_ffi::{c_char, CString};
use super::scheduler::SCHEDULER;
use super::IpcResult;

use core::convert::TryInto;
use libc_binding::stat;

pub fn sys_lstat(filename: *const c_char, buf: *mut stat) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        // Check if given pointers are not bullshit
        let (safe_filename, safe_buf) = {
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
        let mode =
            super::vfs::FilePermissions::from_bits(0o777).expect("file permission creation failed");
        use core::convert::TryFrom;
        // TODO: REMOVE THIS SHIT
        let path = super::vfs::Path::try_from(safe_filename)?;
        // TODO: REMOVE THIS SHIT
        let flags = libc_binding::OpenFlags::O_RDWR;

        let mut current = super::vfs::Current {
            cwd: super::vfs::DirectoryEntryId::new(2),
            uid: 0,
            euid: 0,
            gid: 0,
            egid: 0,
        };

        //TODO: open folow symlink
        let file_operator = match super::vfs::VFS.lock().open(
            &mut current,
            path,
            flags,
            mode, /* access_mode */
        )? {
            IpcResult::Done(file_operator) => file_operator,
            IpcResult::Wait(file_operator, _) => file_operator,
        };
        let mut m = file_operator.lock();
        m.fstat(safe_buf)
    })
}
