#include <sched.h>
#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

extern int errno;

/*
 * fork - create a child process
 */
pid_t fork()
{
	pid_t ret = _user_syscall(CLONE, 2, NULL, 0 /*CLONE_CHILD_CLEARTID|CLONE_CHILD_SETTID|SIGCHLD*/);
	/*
	 * On success, the PID of the child process is returned in the parent,
	 * and 0 is returned in the child.  On failure, -1 is returned in the
	 * parent, no child process is created, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
