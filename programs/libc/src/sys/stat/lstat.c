#include <sys/stat.h>
#include <errno.h>

#warning NOT IMPLEMENTED

int lstat(const char *restrict path, struct stat *restrict buf)
{
	(void)path;
	(void)buf;
	errno = ENOSYS;
	return -1;
}
