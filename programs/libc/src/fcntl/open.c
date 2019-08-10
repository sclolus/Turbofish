#include <fcntl.h>
#include <errno.h>

#warning NOT IMPLEMENTED

int open(const char *path, int oflag, ...)
{
	errno = ENOSYS;
	return -1;
}
