#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>

// The getgid() function shall return the real group ID of the calling
// process. The getgid() function shall not modify errno.

gid_t getgid(void)
{
	TRACE
	return (gid_t)_user_syscall(GETGID, 0);
}
