#include "unistd.h"

extern int user_write(int fd, const char *s, size_t len);
extern int errno;

int write(int fd, const char *s, size_t len)
{
	int ret = user_write(fd, s, len);
	if (ret < 0) {
		errno = -ret;
	}
	return ret;
}
