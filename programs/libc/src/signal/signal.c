
#include "user_syscall.h"
#include "signal.h"

extern int errno;

/*
 * sigaction, rt_sigaction - examine and change a signal action
 */
int sigaction(int signum, const struct sigaction *act, struct sigaction *oldact)
{
	int ret = _user_syscall(SIGACTION, 3, signum, act, oldact);
	/*
	 * sigaction() returns 0 on success; on error, -1 is returned,
	 * and errno is set to indicate the error.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}

/*
 * signal - ANSI C signal handling
 */
sighandler_t signal(int signum, sighandler_t handler)
{
	int ret = _user_syscall(SIGNAL, 2, signum, handler);
	/*
	 * signal() returns the previous value of the signal handler, or SIG_ERR on error.
	 * In the event of an error, errno is set to indicate the cause.
	 */
	if (ret < 0) {
		errno = -ret;
		// TODO: Put SIG_ERR here
		return (sighandler_t)-1;
	} else {
		errno = 0;
		return handler;
	}
}
