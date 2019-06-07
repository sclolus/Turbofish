
#include "user_syscall.h"
#include "unistd.h"

extern int errno;

int mprotect(void *addr, size_t length, int prot)
{
	int ret = _user_syscall(MPROTECT, 3, addr, length, prot);
	if (ret < 0) {
		errno = -ret;
	} else {
		errno = 0;
	}
	return ret;
}
