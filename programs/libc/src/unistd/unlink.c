
#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

/*
 * Delete a name and possibly the file it refers to
 */
int unlink(const char *pathname)
{
	int ret = _user_syscall(UNLINK, 1, pathname);
	/*
	 * On success, zero is returned.  On error, -1 is returned, and errno is set appropriately.
	 */

	set_errno_and_return(ret);
}
