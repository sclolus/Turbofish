#include <unistd.h>
#include <user_syscall.h>

// The geteuid() function shall return the effective user ID of the
// calling process. The geteuid() function shall not modify errno.

uid_t geteuid(void)
{
	uid_t ret = _user_syscall(GETEUID, 0);
	return ret;
}
