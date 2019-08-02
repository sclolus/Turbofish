
#include <signal.h>
#include <errno.h>

int    sigdelset(sigset_t *set, int signo) {
	if (signo < 0 || signo > 31) {
		errno = EINVAL;
		return -1;
	}
	*set &= ~(1 << signo);
	return 0;
}
