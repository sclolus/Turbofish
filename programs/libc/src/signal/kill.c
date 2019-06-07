
#include "user_syscall.h"
#include "signal.h"

extern int errno;

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
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
