
#include "signal.h"
#include "user_syscall.h"

/*
 * getuid - get user identity
 */
uid_t getuid(void)
{
	/*
	 * This function is always successful.
	 */
	return _user_syscall(GETUID, 0);
}
