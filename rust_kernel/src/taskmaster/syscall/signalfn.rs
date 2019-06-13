//! This file contain all the signal related syscall code

use super::SysResult;

use super::process::CpuState;
use super::scheduler::Pid;
use super::scheduler::SCHEDULER;

use errno::Errno;

#[repr(C)]
pub struct Sigaction {}

pub unsafe fn sys_sigaction(_signum: i32, _act: *const Sigaction, _old_act: *mut Sigaction) -> SysResult<u32> {
    // unpreemptible_context!({
        unimplemented!();
    // })
}

pub unsafe fn sys_kill(pid: Pid, signum: u32, cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();
        let curr_process_pid = scheduler.curr_process_pid();
        let task = scheduler.get_process_mut(pid).ok_or(Errno::Esrch)?;
        let res = task.signal.kill(signum)?;
        // if this is the current process, deliver the signal
        if res == 0 && pid == curr_process_pid {
        (*cpu_state).registers.eax = res;
            task.signal.has_pending_signals();
        }
        Ok(res)
    })
}

pub unsafe fn sys_signal(signum: u32, handler: extern "C" fn(i32)) -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER.lock().curr_process_mut().signal.signal(signum, handler)
    })
}

pub unsafe fn sys_sigreturn(cpu_state: *mut CpuState) -> SysResult<u32> {
    unpreemptible_context!({
        SCHEDULER.lock().curr_process_mut().signal.sigreturn(cpu_state)
    })
}
