
#include "user_syscall.h"
#include "sys/mman.h"
#include "stdio.h"

extern int errno;

struct mmap_struct {
	void *addr;
	size_t length;
	int prot;
	int flags;
	int fd;
	off_t offset;
};

void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset)
{
	struct mmap_struct s = {
		addr = addr,
		length = length,
		prot = prot,
		flags = flags,
		fd = fd,
		offset = offset,
	};
	void *ret = (void *)_user_syscall(MMAP, 1, &s);

	s8 err = (u32)ret & 0x7f;
	if (err != 0) {
		errno = err;
		return (void *)MAP_FAILED;
	} else {
		errno = 0;
		return ret;
	}
}
