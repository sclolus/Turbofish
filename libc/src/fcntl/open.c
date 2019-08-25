#include <user_syscall.h>
#include <fcntl.h>
#include <errno.h>

/*
 * open and possibly create a file
 */
int open(const char *path, int oflag, ...)
{
	// TODO: Manage with the ... variadic
	int ret = _user_syscall(OPEN, 2, path, oflag);
	/*
	 * open() return the new file descriptor, or -1 if an error
	 * occurred (in which case, errno is set appropriately)
	 */
	set_errno_and_return(ret);
}

#include <custom.h>

#warning NOT IMPLEMENTED

int open64(const char *path, int oflag, ...)
{
	DUMMY
	(void)path;
	(void)oflag;
	errno = ENOSYS;
	return -1;
}
