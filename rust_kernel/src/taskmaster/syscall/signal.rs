use super::SysResult;

use super::scheduler::SCHEDULER;
use super::signal_interface::StructSigaction;

use core::convert::TryInto;
use errno::Errno;
/// Register a new handler for a specified signum
pub unsafe fn sys_signal(signum: u32, handler: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let s: StructSigaction = StructSigaction {
            sa_handler: handler,
            sa_mask: Default::default(),
            sa_flags: Default::default(),
            sa_restorer: 0,
        };

        let mut scheduler = SCHEDULER.lock();
        let struct_sigaction = scheduler
            .current_task_mut()
            .signal
            .new_handler(signum.try_into().map_err(|_| Errno::Einval)?, &s)?;
        Ok(struct_sigaction.sa_handler as u32)
    })
}
