#include <ltrace.h>
#include <user_syscall.h>
#include <signal.h>
#include <errno.h>

/*
 * signal - ANSI C signal handling
 */
sighandler_t signal(int signum, sighandler_t handler)
{
	TRACE
	int ret = _user_syscall(SIGNAL, 2, signum, handler);
	/*
	 * signal() returns the previous value of the signal handler, or SIG_ERR on error.
	 * In the event of an error, errno is set to indicate the cause.
	 */
	if (ret < 0) {
		errno = -ret;
		return (sighandler_t)-1;
	} else {
		return (sighandler_t)ret;
	}
}
