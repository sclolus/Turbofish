#warning NOT IMPLEMENTED

#include <sys/stat.h>

#include <errno.h>
#include <custom.h>

int stat64(const char *restrict pathname, struct stat64 *restrict stat)
{
	DUMMY
	(void)pathname;
	(void)stat;
	errno = ENOSYS;
	return -1;
}

int lstat64(const char *restrict pathname, struct stat64 *restrict stat)
{
	DUMMY
	(void)pathname;
	(void)stat;
	errno = ENOSYS;
	return -1;
}

int fstat64(int fd, struct stat64 *stat)
{
	DUMMY
	(void)fd;
	(void)stat;
	errno = ENOSYS;
	return -1;
}
