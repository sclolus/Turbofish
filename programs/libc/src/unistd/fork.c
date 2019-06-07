
#include "unistd.h"

extern pid_t user_fork();
extern int errno;

/*
 * fork - create a child process
 */
pid_t fork()
{
	pid_t ret = user_fork();
	/*
	 * On success, the PID of the child process is returned in the parent,
	 * and 0 is returned in the child.  On failure, -1 is returned in the
	 * parent, no child process is created, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -(int)ret;
		return -1;
	} else {
		errno = 0;
		return ret;
	}
}
