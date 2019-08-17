#include <unistd.h>
#include <user_syscall.h>

// The geteuid() function shall return the effective user ID of the
// calling process. The geteuid() function shall not modify errno.

uid_t geteuid(void)
{
	return (uid_t)_user_syscall(GETEUID, 0);
}
