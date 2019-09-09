#include <ltrace.h>
#include <user_syscall.h>
#include <time.h>
#include <errno.h>

int nanosleep(const struct timespec *req, struct timespec *rem)
{
	TRACE
	int ret = _user_syscall(NANOSLEEP, 2, req, rem);
	set_errno_and_return(ret);
}
