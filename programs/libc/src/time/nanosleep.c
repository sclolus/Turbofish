
#include <user_syscall.h>
#include <time.h>
#include <errno.h>

int nanosleep(const struct timespec *req, struct timespec *rem) {
	int ret = _user_syscall(NANOSLEEP, 2, req, rem);
	set_errno_and_return(ret);
}
