
#include "user_syscall.h"
#include "sys/mman.h"

extern int errno;

int munmap(void *addr, size_t length)
{
	int ret = _user_syscall(MUNMAP, 2, addr, length);
	if (ret < 0) {
		errno = -ret;
	} else {
		errno = 0;
	}
	return ret;
}
