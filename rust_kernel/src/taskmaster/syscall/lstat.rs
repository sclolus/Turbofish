use super::SysResult;

use super::scheduler::SCHEDULER;
use super::IpcResult;
use libc_binding::c_char;

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

            (
                v.make_checked_str(filename)?,
                v.make_checked_ref_mut::<stat>(buf)?,
            )
        };
        let mode =
            libc_binding::FileType::from_bits(0o777).expect("file permission creation failed");
        use core::convert::TryFrom;
        let path = super::vfs::Path::try_from(safe_filename)?;
        let flags = libc_binding::OpenFlags::empty();

        let tg = scheduler.current_thread_group();
        let creds = &tg.credentials;
        let cwd = &tg.cwd;
        //TODO: open folow symlink
        let file_operator = match super::vfs::VFS.lock().open(cwd, creds, path, flags, mode)? {
            IpcResult::Done(file_operator) => file_operator,
            IpcResult::Wait(file_operator, _) => file_operator,
        };
        let mut m = file_operator.lock();
        m.fstat(safe_buf)
    })
}
