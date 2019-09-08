
#include <ltrace.h>
#include <signal.h>
#include <errno.h>

int    sigismember(const sigset_t *set, int signo) {
	if (signo < 0 || signo > 31) {
		errno = EINVAL;
		return -1;
	}
	return (*set & (1 << signo)) != 0;
}
