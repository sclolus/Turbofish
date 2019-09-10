use super::scheduler::{unpreemptible, SCHEDULER};
use super::thread_group::Status;
use super::SysResult;

/// [CX] [Option Start] Process termination caused by any reason shall
/// have the following consequences: [Option End]
///
/// Note: These consequences are all extensions to the ISO C standard
///  and are not further CX shaded. However, functionality relating to
///  the XSI option is shaded.
///
///  All of the file descriptors, directory streams, conversion
///  descriptors, and message catalog descriptors open in the calling
///  process shall be closed.
///
///  [XSI] [Option Start] If the parent process of the calling process
///  has set its SA_NOCLDWAIT flag or has set the action for the
///  SIGCHLD signal to SIG_IGN:
///
///      The process' status information (see Status Information), if
///      any, shall be discarded.
///
///      The lifetime of the calling process shall end immediately. If
///      SA_NOCLDWAIT is set, it is implementation-defined whether a
///      SIGCHLD signal is sent to the parent process.
///
///      If a thread in the parent process of the calling process is
///      blocked in wait(), waitpid(), or waitid(), and the parent
///      process has no remaining child processes in the set of
///      waited-for children, the wait(), waitid(), or waitpid()
///      function shall fail and set errno to [ECHILD].
///
///  Otherwise: [Option End]
///
///      Status information (see Status Information) shall be
///      generated.
///
///      The calling process shall be transformed into a zombie
///      process. Its status information shall be made available to
///      the parent process until the process' lifetime ends.
///
///      The process' lifetime shall end once its parent obtains the
///      process' status information via a currently-blocked or future
///      call to wait(), waitid() (without WNOWAIT), or waitpid().
///
///      If one or more threads in the parent process of the calling
///      process is blocked in a call to wait(), waitid(), or
///      waitpid() awaiting termination of the process, one (or, if
///      any are calling waitid() with WNOWAIT, possibly more) of
///      these threads shall obtain the process' status information as
///      specified in Status Information and become unblocked.
///
///      A SIGCHLD shall be sent to the parent process.
///
///  Termination of a process does not directly terminate its
///  children. The sending of a SIGHUP signal as described below
///  indirectly terminates children in some circumstances.
///
///  The parent process ID of all of the existing child processes and
///  zombie processes of the calling process shall be set to the
///  process ID of an implementation-defined system process. That is,
///  these processes shall be inherited by a special system process.
///
///  [XSI] [Option Start] Each attached shared-memory segment is
///  detached and the value of shm_nattch (see shmget()) in the data
///  structure associated with its shared memory ID shall be
///  decremented by 1. [Option End]
///
///  [XSI] [Option Start] For each semaphore for which the calling
///  process has set a semadj value (see semop()), that value shall be
///  added to the semval of the specified semaphore. [Option End]
///
// TODO:
///  If the process is a controlling process, the SIGHUP signal shall
///  be sent to each process in the foreground process group of the
///  controlling terminal belonging to the calling process.
///
///  If the process is a controlling process, the controlling terminal
///  associated with the session shall be disassociated from the
///  session, allowing it to be acquired by a new controlling process.
///
///  If the exit of the process causes a process group to become
///  orphaned, and if any member of the newly-orphaned process group
///  is stopped, then a SIGHUP signal followed by a SIGCONT signal
///  shall be sent to each process in the newly-orphaned process
///  group.
///
///  All open named semaphores in the calling process shall be closed
///  as if by appropriate calls to sem_close().
///
///  [ML] [Option Start] Any memory locks established by the process
///  via calls to mlockall() or mlock() shall be removed. If locked
///  pages in the address space of the calling process are also mapped
///  into the address spaces of other processes and are locked by
///  those processes, the locks established by the other processes
///  shall be unaffected by the call by this process to _Exit() or
///  _exit(). [Option End]
///
///  Memory mappings that were created in the process shall be
///  unmapped before the process is destroyed.
///
///  [TYM] [Option Start] Any blocks of typed memory that were mapped
///  in the calling process shall be unmapped, as if munmap() was
///  implicitly called to unmap them. [Option End]
///
///  [MSG] [Option Start] All open message queue descriptors in the
///  calling process shall be closed as if by appropriate calls to
///  mq_close(). [Option End]
///
///  Any outstanding cancelable asynchronous I/O operations may be
///  canceled. Those asynchronous I/O operations that are not canceled
///  shall complete as if the _Exit() or _exit() operation had not yet
///  occurred, but any associated signal notifications shall be
///  suppressed. The _Exit() or _exit() operation may block awaiting
///  such I/O completion. Whether any I/O is canceled, and which I/O
///  may be canceled upon _Exit() or _exit(), is
///  implementation-defined.
///
///  Threads terminated by a call to _Exit() or _exit() shall not
///  invoke their cancellation cleanup handlers or per-thread data
///  destructors.
///
///  [OB TRC] [Option Start] If the calling process is a trace
///  controller process, any trace streams that were created by the
///  calling process shall be shut down as described by the
///  posix_trace_shutdown() function, and mapping of trace event names
///  to trace event type identifiers of any process built for these
///  trace streams may be deallocated. [Option End]

pub unsafe fn sys_exit(status: i32) -> SysResult<u32> {
    // Avoid preempting when we are un the exit routine
    unpreemptible();
    SCHEDULER
        .lock()
        .current_thread_group_exit(Status::Exited(status));
    Ok(0)
}
