
#include <user_syscall.h>
#include <signal.h>
#include <errno.h>

/*
 * kill - send signal to a process
 */
int kill(pid_t pid, int sig)
{
	int ret = _user_syscall(KILL, 2, pid, sig);
	/*
	 * On success (at least one signal was sent), zero is returned.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
