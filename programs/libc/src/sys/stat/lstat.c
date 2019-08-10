#include <sys/stat.h>
#include <errno.h>

int lstat(const char *restrict path, struct stat *restrict buf)
{
	errno = ENOSYS;
	return -1;
}
