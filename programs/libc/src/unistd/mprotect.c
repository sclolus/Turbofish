#include "unistd.h"

extern int user_mprotect(void *addr, size_t length, int prot);
extern int errno;

int mprotect(void *addr, size_t length, int prot)
{
	int ret = user_mprotect(addr, length, prot);
	if (ret < 0) {
		errno = -ret;
	} else {
		errno = 0;
	}
	return ret;
}
