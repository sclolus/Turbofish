//! waitpid (wait) implementations

use super::scheduler::SCHEDULER;
use super::scheduler::{auto_preempt, uninterruptible};
use super::task::{ProcessState, WaitingState};
use super::SysResult;

use errno::Errno;

fn waitpid(pid: i32, wstatus: *mut i32, options: i32) -> SysResult<u32> {
    let mut scheduler = SCHEDULER.lock();

    let v = &mut scheduler.curr_process_mut().unwrap_running_mut().virtual_allocator;

    // If wstatus is not NULL, wait() and waitpid() store status information in the int to which it points.
    // If the given pointer is a bullshit pointer, wait() and waitpid() return EFAULT
    if wstatus != 0x0 as *mut i32 {
        v.check_user_ptr::<i32>(wstatus)?;
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

    let task = scheduler.curr_process();

    // Check if the child is already dead: Return his PID if true or NONE
    // Errno: Return ECHILD if not child or child PID specified is wrong
    let child_pid = if pid < 0 {
        // Check if at leat one child exists
        if task.child.len() == 0 {
            return Err(Errno::Echild);
        }
        // Check is the at least one child is a already a zombie -> Return immediatly child PID
        if let Some(&zombie_pid) = task
            .child
            .iter()
            .find(|current_pid| scheduler.all_process.get(current_pid).expect("Pid must be here").is_zombie())
        {
            Some(zombie_pid)
        } else {
            None
        }
    } else {
        // Check if specified child exists
        if let Some(elem) = task.child.iter().find(|&&current_pid| current_pid == pid as u32) {
            if scheduler.all_process.get(elem).expect("Pid must be here").is_zombie() {
                Some(*elem)
            } else {
                None
            }
        } else {
            return Err(Errno::Echild);
        }
    };

    match child_pid {
        Some(pid) => {
            let child = scheduler.all_process.get(&pid).expect("Pid must be here");
            // TODO: Manage terminated value with signal
            if wstatus != 0x0 as *mut i32 {
                unsafe {
                    *wstatus = match child.process_state {
                        ProcessState::Zombie(status) => status,
                        _ => panic!("WTF"),
                    };
                }
            }
            // fflush zombie
            scheduler.all_process.remove(&pid).expect("Pid must be here");
            let task = scheduler.curr_process_mut();
            task.child.remove_item(&pid).unwrap();
            // Return immediatly
            Ok(pid)
        }
        None => {
            // Set process as Waiting for ChildDeath. set the PID option inside
            scheduler
                .curr_process_mut()
                .set_waiting(WaitingState::ChildDeath(if pid < 0 { None } else { Some(pid as u32) }, 0));

            // Auto-preempt calling
            let ret = auto_preempt();

            // Re-Lock immediatly critical ressources (auto_preempt unlocked all)
            uninterruptible();
            let mut scheduler = SCHEDULER.lock();

            if ret < 0 {
                // Reset as running
                scheduler.curr_process_mut().set_running();
                return Err(Errno::Eintr);
            } else {
                let child_pid = match &scheduler.curr_process().process_state {
                    // Read the fields of the WaintingState::ChildDeath(x, y)
                    ProcessState::Waiting(_, WaitingState::ChildDeath(opt, status)) => {
                        // Set wstatus pointer is not null by reading y
                        if wstatus != 0x0 as *mut i32 {
                            unsafe {
                                *wstatus = *status as i32;
                            }
                        }
                        let t = opt.expect("Cannot be None");
                        scheduler.all_process.remove(&t).expect("Pid must be here");
                        t
                    }
                    _ => panic!("WTF"),
                };
                // Set process as Running, Set return readen value in Ok(x)
                scheduler.curr_process_mut().set_running();
                let task = scheduler.curr_process_mut();
                task.child.remove_item(&child_pid).unwrap();
                Ok(child_pid)
            }
        }
    }
}

pub fn sys_waitpid(pid: i32, wstatus: *mut i32, options: i32) -> SysResult<u32> {
    uninterruptible_context!({ waitpid(pid, wstatus, options) })
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

// On exit() fn
// eprintln!("exiting {:?}", self.curr_process());
// Get the current process's PID
// let p = self.curr_process();
// if let Some(father_pid) = p.parent {
//     let father = self.all_process.get_mut(&father_pid).expect("process parent should exist");
//     if father.is_waiting() {
//         self.running_process.try_push(father_pid).unwrap();
//         // dbg!("exit father set running");
//         father.set_running();
//     }
// }
