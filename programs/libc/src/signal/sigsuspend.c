#include <signal.h>
#include <errno.h>
#include <user_syscall.h>

/*
 * sigsuspend - wait for a signal
 */
int sigsuspend(const sigset_t *sigmask)
{
	int ret = _user_syscall(SIGSUSPEND, 1, sigmask);

	set_errno_and_return(ret);
}
