//! sys_nanosleep implementation

use super::SysResult;

use super::scheduler::auto_preempt;
use super::scheduler::SCHEDULER;
use super::thread::WaitingState;

use libc_binding::Errno;

use crate::drivers::PIT0;
use crate::math::convert::Convert;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct TimeSpec {
    tv_sec: u32,
    tv_nsec: i32,
}

extern "C" {
    fn _get_pit_time() -> u32;
}

/// The nanosleep() function shall cause the current thread to be
/// suspended from execution until either the time interval specified
/// by the rqtp argument has elapsed or a signal is delivered to the
/// calling thread, and its action is to invoke a signal-catching
/// function or to terminate the process. The suspension time may be
/// longer than requested because the argument value is rounded up to
/// an integer multiple of the sleep resolution or because of the
/// scheduling of other activity by the system. But, except for the
/// case of being interrupted by a signal, the suspension time shall
/// not be less than the time specified by rqtp, as measured by the
/// system clock CLOCK_REALTIME.
///
/// The use of the nanosleep() function has no effect on the action or
/// blockage of any signal.  If the nanosleep() function returns
/// because the requested time has elapsed, its return value shall be
/// zero.
///
/// If the nanosleep() function returns because it has been
/// interrupted by a signal, it shall return a value of -1 and set
/// errno to indicate the interruption. If the rmtp argument is
/// non-NULL, the timespec structure referenced by it is updated to
/// contain the amount of time remaining in the interval (the
/// requested time minus the time actually slept). The rqtp and rmtp
/// arguments can point to the same object. If the rmtp argument is
/// NULL, the remaining time is not returned.
///
/// If nanosleep() fails, it shall return a value of -1 and set errno
/// to indicate the error.
fn nanosleep(req: *const TimeSpec, rem: *mut TimeSpec) -> SysResult<u32> {
    let mut scheduler = SCHEDULER.lock();

    let v = scheduler
        .current_thread_mut()
        .unwrap_process_mut()
        .get_virtual_allocator();

    let req = v.make_checked_ref(req)?;
    let safe_rem = if rem.is_null() {
        None
    } else {
        Some(v.make_checked_ref_mut(rem)?)
    };

    // drop the mutex
    drop(v);

    let nsec = req.tv_nsec;
    if nsec < 0 || nsec >= 1000000000 {
        return Err(Errno::EINVAL);
    }

    // Set precision as 1/1000
    let request_time = req.tv_sec as f32 + (req.tv_nsec / 1000000) as f32 / 1000.;
    let pit_period = 1. / PIT0.lock().get_frequency().expect("PIT0 not initialized");
    let next_wake = (request_time / pit_period) as u32 + unsafe { _get_pit_time() };

    // Set as Sleeping
    scheduler
        .current_thread_mut()
        .set_waiting(WaitingState::Sleeping(next_wake));

    // auto preemption mechanism set environement as preemptible
    match auto_preempt() {
        Err(Errno::EINTR) => {
            let now = unsafe { _get_pit_time() };
            if now < next_wake {
                let remaining_time = (next_wake - now) as f32 * pit_period;

                if let Some(rem) = safe_rem {
                    rem.tv_sec = remaining_time.trunc() as u32;
                    rem.tv_nsec = ((remaining_time * 1000.).trunc() as u32 % 1000 * 1000000) as i32;
                }
            }
            Err(Errno::EINTR)
        }
        _ => Ok(0),
    }
}

pub fn sys_nanosleep(req: *const TimeSpec, rem: *mut TimeSpec) -> SysResult<u32> {
    unpreemptible_context!({ nanosleep(req, rem) })
}
