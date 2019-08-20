#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

/*
 * Close a file descriptor
 */
int close(int fd)
{
	int ret = _user_syscall(CLOSE, 1, fd);
	/*
	 * close() returns zero on success.  On error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
