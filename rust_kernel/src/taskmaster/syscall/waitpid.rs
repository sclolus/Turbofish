//! waitpid (wait) implementations

use super::scheduler::Pid;
use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, interruptible, uninterruptible};
use super::tools::check_user_ptr;
use super::SysResult;

use errno::Errno;

fn waitpid(pid: i32, wstatus: *mut i32, options: i32) -> SysResult<u32> {
    let mut scheduler = SCHEDULER.lock();

    let v = &mut scheduler.curr_process_mut().unwrap_running_mut().virtual_allocator;

    // If wstatus is not NULL, wait() and waitpid() store status information in the int to which it points.
    // If the given pointer is a bullshit pointer, wait() and waitpid() return EFAULT
    if wstatus != 0x0 as *mut i32 {
        check_user_ptr::<i32>(wstatus, v)?;
    }

    // WIFEXITED(wstatus)
    // returns true if the child terminated normally, that is, by calling exit(3) or _exit(2), or by returning from main().

    // WEXITSTATUS(wstatus)
    // returns  the exit status of the child. This consists of the least significant 8 bits of the status argument that
    // the child specified in a call to exit(3) or _exit(2) or as the argument for a
    // return statement in main().  This macro should be employed only if WIFEXITED returned true.

    // the two next macro are signal dedicated ... WIFSIGNALED(wstatus) && WTERMSIG(wstatus)

    // Return EINVAL for any unknown option
    // TODO: Code at least WNOHANG and WUNTRACED for Posix
    if options != 0 {
        return Err(Errno::Einval);
    }

    // Return ECHILD if not child or child PID specified is wrong
    if pid < 0 {
        // Check if at leat one child exists
    } else {
        // Check if specified child exists
    }

    // Can be mixed ...

    if pid < 0 {
        // In case of PID < 0, Check is the at least one child is a already a zombie -> Return immediatly child PID
        // fflush zombie
    } else {
        // In case of PID >= 0, Check is specified child PID is already a zombie -> Return immediatly child PID
        // fflush zombie
    }

    // Set process as Waiting for ChildDeath. set the PID option inside

    // Auto-preempt calling
    auto_preempt();

    // Read the fields of the WaintingState::ChildDeath(x, y)
    // Set wstatus pointer is not null by reading y
    // Set process as Running, Set return readen value in Ok(x)
    Ok(0)
}

pub fn sys_waitpid(pid: i32, wstatus: *mut i32, options: i32) -> SysResult<u32> {
    uninterruptible();
    let res = waitpid(pid, wstatus, options);
    interruptible();
    res
}

// TODO: Solve Gloubiboulga
// pub fn wait(&mut self) -> SysResult<Pid> {
//     let mut p = self.all_process.remove(&self.curr_process_pid).unwrap();
//     if p.child.is_empty() {
//         return Err(Errno::Echild);
//     }
//     TODO: Solve Borrow
//         if let None = p.child.iter().find(|c| self.all_process.get(c).unwrap().is_zombie()) {
//         p.set_waiting(WaitingState::ChildDeath(None));
//         // dbg!("set waiting");
//         self.all_process.insert(self.curr_process_pid, p);
//         self.remove_curr_running();
//         auto_preempt();
//         dbg!("return to live after schedule");
//     }
//     if let Some(child) = p.child.iter().find(|c| self.all_process.get(c).unwrap().is_zombie()) {
//         child.exit_status
//     }
// }
