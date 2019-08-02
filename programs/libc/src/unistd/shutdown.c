
#include "user_syscall.h"

extern int errno;

/*
 * Power off the computer
 */
int shutdown(void)
{
	int ret = _user_syscall(SHUTDOWN, 0);
	/*
	 * On Error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return ret;
	}
}
