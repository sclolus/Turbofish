#include <sys/stat.h>
#include <errno.h>

#warning NOT IMPLEMENTED
#include <custom.h>

int fstat(int fildes, struct stat *buf)
{
	DUMMY
	(void)fildes;
	(void)buf;
	errno = ENOSYS;
	return -1;
}

int fstat64(int fildes, struct stat *buf)
{
	DUMMY
	(void)fildes;
	(void)buf;
	errno = ENOSYS;
	return -1;
}
