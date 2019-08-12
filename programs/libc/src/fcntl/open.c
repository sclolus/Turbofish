#include <fcntl.h>
#include <errno.h>

#warning NOT IMPLEMENTED

int open(const char *path, int oflag, ...)
{
	(void)path;
	(void)oflag;
	errno = ENOSYS;
	return -1;
}

int open64(const char *path, int oflag, ...)
{
	(void)path;
	(void)oflag;
	errno = ENOSYS;
	return -1;
}
