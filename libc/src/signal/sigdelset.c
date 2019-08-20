#include <signal.h>
#include <errno.h>

/*
 * POSIX signal set operations
 * sigdelset() del signal signum from set.
 * return 0 on success and -1 on error.
 */
int    sigdelset(sigset_t *set, int signo)
{
	if (signo < 0 || signo > 31) {
		errno = EINVAL;
		return -1;
	}
	*set &= ~(1 << signo);
	return 0;
}
