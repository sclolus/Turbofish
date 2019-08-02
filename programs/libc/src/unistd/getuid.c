#include <signal.h>
#include <user_syscall.h>
#include <errno.h>

/*
 * getuid - get user identity
 */
uid_t getuid(void)
{
	/*
	 * This function is always successful.
	 */
	uid_t ret = _user_syscall(GETUID, 0);
	set_errno_and_return(ret);
}
