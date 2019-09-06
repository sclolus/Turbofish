use super::SysResult;

use super::scheduler::{Scheduler, SCHEDULER};
use super::vfs::{Path, VFS};
use super::IpcResult;
use core::convert::TryFrom;
use libc_binding::c_char;
use libc_binding::stat;

pub fn statfn(scheduler: &Scheduler, path: Path, buf: &mut stat) -> SysResult<u32> {
    let mode = libc_binding::FileType::from_bits(0o777).expect("file permission creation failed");
    // TODO: REMOVE THIS SHIT
    // TODO: REMOVE THIS SHIT
    let flags = libc_binding::OpenFlags::O_RDWR;

    let tg = scheduler.current_thread_group();
    let creds = &tg.credentials;
    let cwd = &tg.cwd;
    let file_operator = match VFS.lock().open(cwd, creds, path, flags, mode)? {
        IpcResult::Done(file_operator) => file_operator,
        IpcResult::Wait(file_operator, _) => file_operator,
    };
    let mut m = file_operator.lock();
    m.fstat(buf)
}

pub fn sys_stat(filename: *const c_char, buf: *mut stat) -> SysResult<u32> {
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
        let path = Path::try_from(safe_filename)?;
        statfn(&scheduler, path, safe_buf)
    })
}
