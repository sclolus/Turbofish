#include <sys/stat.h>
#include <errno.h>

#warning NOT IMPLEMENTED
#include <custom.h>

int lstat(const char *restrict path, struct stat *restrict buf)
{
	DUMMY
	(void)path;
	(void)buf;
	errno = ENOSYS;
	return -1;
}

int lstat64(const char *restrict path, struct stat *restrict buf)
{
	DUMMY
	(void)path;
	(void)buf;
	errno = ENOSYS;
	return -1;
}
