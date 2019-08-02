
#include "user_syscall.h"
#include "time.h"

extern int errno;

int nanosleep(const struct timespec *req, struct timespec *rem) {
	int ret = _user_syscall(NANOSLEEP, 2, req, rem);
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}
