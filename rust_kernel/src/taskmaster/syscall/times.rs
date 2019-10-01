use super::scheduler::SCHEDULER;
use super::SysResult;
use libc_binding::tms;

use core::ptr;

// DUMMY IMPLEMENTATION BUT HERE FUCK YOU DASH.
fn times(buf: Option<&mut tms>) -> SysResult<u32> {
    if let Some(buf) = buf {
        *buf = tms {
            tms_utime: 42,
            tms_stime: 42,
            tms_cutime: 42,
            tms_cstime: 42,
        };
    }
    let mut clocks = 42;

    unsafe {
        asm!("rdtsc\nmov eax, $0" : "=*m"(&mut clocks as *mut _):: "memory" : "intel", "volatile");
    }
    Ok(clocks)
}

pub fn sys_times(buf: *mut tms) -> SysResult<u32> {
    unpreemptible_context!({
        let scheduler = SCHEDULER.lock();

        let buf = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            if buf == ptr::null_mut() {
                None
            } else {
                Some(v.make_checked_ref_mut(buf)?)
            }
        };

        // let tg = scheduler.current_thread_group_mut();
        times(buf)
    })
}
