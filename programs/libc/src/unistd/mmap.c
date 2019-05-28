#include "unistd.h"
#include "stdio.h"

extern void *user_mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);

void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset)
{
	void *ret = user_mmap(addr, length, prot, flags, fd, offset);

	printf("mmap return %x\n", ret);
	s8 err = (u32)ret & 0x7f;
	if (err != 0) {
		// TODO: Set global ERRNO
		return (void *)MAP_FAILED;
	} else {
		return ret;
	}
}
