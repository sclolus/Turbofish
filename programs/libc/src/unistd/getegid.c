#include <unistd.h>
#include <user_syscall.h>

// The getegid() function shall return the effective group ID of the
// calling process. The getegid() function shall not modify errno.

gid_t getegid(void)
{
	return (gid_t)_user_syscall(GETEGID, 0);
}
