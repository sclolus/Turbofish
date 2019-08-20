#include <user_syscall.h>
#include <signal.h>
#include <errno.h>

/*
 * sigaction - examine and change a signal action
 */
int sigaction(int signum, const struct sigaction *act, struct sigaction *oldact)
{
	int ret = _user_syscall(SIGACTION, 3, signum, act, oldact);
	/*
	 * sigaction() returns 0 on success; on error, -1 is returned,
	 * and errno is set to indicate the error.
	 */
	set_errno_and_return(ret);
}
