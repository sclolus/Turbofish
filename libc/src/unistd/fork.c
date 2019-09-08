#include <ltrace.h>
#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

/*
 * fork - create a child process
 */
pid_t fork()
{
	TRACE
	int ret = _user_syscall(FORK, 0);
	/*
	 * On success, the PID of the child process is returned in the parent,
	 * and 0 is returned in the child.  On failure, -1 is returned in the
	 * parent, no child process is created, and errno is set appropriately.
	 */

	set_errno_and_return(ret);
}
