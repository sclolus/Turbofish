
#include "user_syscall.h"
#include "unistd.h"

extern int errno;

/*
 * Close a file descriptor
 */
int close(int fd)
{
	int ret = _user_syscall(CLOSE, 1, fd);
	/*
	 * close() returns zero on success.  On error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
