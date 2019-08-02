
#include <user_syscall.h>
#include <errno.h>

/*
 * Power off the computer
 */
int shutdown(void)
{
	int ret = _user_syscall(SHUTDOWN, 0);
	/*
	 * On Error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
