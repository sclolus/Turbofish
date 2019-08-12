#include <sys/stat.h>
#include <errno.h>

#warning NOT IMPLEMENTED

int fstat(int fildes, struct stat *buf)
{
	(void)fildes;
	(void)buf;
	errno = ENOSYS;
	return -1;
}
