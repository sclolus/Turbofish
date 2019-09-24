#include <unistd.h>
#include <errno.h>

#warning DUMMY IMPLEMENTATION of sysconf.
long sysconf(int name)
{
	errno = ENOSYS;
	return -1L;
}
