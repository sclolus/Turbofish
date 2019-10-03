#include <ltrace.h>
#include <stddef.h>
#include <sys/resource.h>
#include <sys/select.h>
#include <wait.h>
#include <user_syscall.h>
#include <errno.h>

/*
 * wait3, wait4 - wait for process to change state, BSD style
 */
pid_t wait4(pid_t pid, int *wstatus, int options, struct rusage *rusage)
{
	TRACE
	pid_t ret = _user_syscall(WAIT4, 4, pid, wstatus, options, rusage);
	/*
	 * on success, returns the process ID of the child whose state
	 * has changed; if WNOHANG was specified and one or more child(ren)
	 * specified by pid exist, but have not yet changed state, then 0
	 * is returned.  On error, -1 is returned.
	 * If rusage is not NULL, the struct rusage to which it points will be
	 * filled with accounting information about the child.  See getrusage(2)
	 * for details
	 */
	set_errno_and_return(ret);
}
