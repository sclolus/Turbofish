
#include "user_syscall.h"
#include "unistd.h"

extern int errno;

int write(int fd, const void *s, size_t len)
{
	int ret = _user_syscall(WRITE, 3, fd, s, len);
	if (ret < 0) {
		errno = -ret;
	} else {
		errno = 0;
	}
	return ret;
}
