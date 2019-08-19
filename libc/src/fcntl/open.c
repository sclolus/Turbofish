#include <fcntl.h>
#include <errno.h>

#warning NOT IMPLEMENTED
#include <custom.h>

int open(const char *path, int oflag, ...)
{
	DUMMY
	(void)path;
	(void)oflag;
	errno = ENOSYS;
	return -1;
}

int open64(const char *path, int oflag, ...)
{
	DUMMY
	(void)path;
	(void)oflag;
	errno = ENOSYS;
	return -1;
}
