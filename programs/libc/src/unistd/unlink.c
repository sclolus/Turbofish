
#include "user_syscall.h"
#include "unistd.h"

extern int errno;

/*
 * Delete a name and possibly the file it refers to
 */
int unlink(const char *pathname)
{
	int ret = _user_syscall(UNLINK, 1, pathname);
	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
