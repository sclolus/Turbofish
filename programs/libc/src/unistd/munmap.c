#include "unistd.h"

extern int user_munmap(void *addr, size_t length);
extern int errno;

int munmap(void *addr, size_t length)
{
	int ret = user_munmap(addr, length);
	if (ret < 0) {
		errno = -ret;
	} else {
		errno = 0;
	}
	return ret;
}
