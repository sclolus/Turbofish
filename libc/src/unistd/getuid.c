#include <ltrace.h>
#include <signal.h>
#include <user_syscall.h>

/// The getuid() function shall return the real user ID of the calling
/// process. The getuid() function shall not modify errno.

uid_t getuid(void)
{
	TRACE
	/*
	 * This function is always successful.
	 */
	return (uid_t)_user_syscall(GETUID, 0);
}
