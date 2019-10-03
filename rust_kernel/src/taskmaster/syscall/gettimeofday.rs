use super::kmodules::CURRENT_UNIX_TIME;
use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::{timeval, timezone};

use core::ptr;
use core::sync::atomic::Ordering;

fn gettimeofday(timeval: Option<&mut timeval>, _timezone: Option<&mut timezone>) -> SysResult<u32> {
    let current_unix_time = unsafe { CURRENT_UNIX_TIME.load(Ordering::Acquire) };

    if let Some(timeval) = timeval {
        timeval.tv_sec = current_unix_time as i32;
        timeval.tv_usec = 0; // No fucks given.
    }
    Ok(0)
}

pub fn sys_gettimeofday(timeval: *mut timeval, timezone: *mut timezone) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = scheduler
            .current_thread_mut()
            .unwrap_process_mut()
            .get_virtual_allocator();

        let timeval = if timeval != ptr::null_mut() {
            Some(v.make_checked_ref_mut(timeval)?)
        } else {
            None
        };

        let timezone = if timezone != ptr::null_mut() {
            Some(v.make_checked_ref_mut(timezone)?)
        } else {
            None
        };

        gettimeofday(timeval, timezone)
    })
}
