#include <signal.h>
#include <errno.h>
#include <user_syscall.h>

/*
 * sigprocmask - examine and change blocked signals
 */
int sigprocmask(int how, const sigset_t *restrict set,
				sigset_t *restrict oset)
{
	if (how != SIG_BLOCK && how != SIG_UNBLOCK && how != SIG_SETMASK) {
		return EINVAL;
	}
	int ret = _user_syscall(SIGPROCMASK, 3, how, set, oset);
	set_errno_and_return(ret);
}
