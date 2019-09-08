#include <ltrace.h>
#include <user_syscall.h>
#include <errno.h>

/*
 * Reboot the computer
 */
int reboot(void)
{
	TRACE
	int ret = _user_syscall(REBOOT, 0);
	/*
	 * On Error, -1 is returned, and errno is set appropriately.
	 */
	set_errno_and_return(ret);
}
