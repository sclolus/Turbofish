
#include "signal.h"

extern int user_kill(pid_t pid, int sig);
extern int errno;

/*
 * kill - send signal to a process
 */
int kill(pid_t pid, int sig)
{
	int ret = user_kill(pid, sig);
	/*
	 * On success (at least one signal was sent), zero is returned.
	 * On error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		return 0;
	}
}
