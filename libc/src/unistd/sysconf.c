#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION of sysconf.
long sysconf(int name)
{
	TRACE
	DUMMY
	errno = ENOSYS;
	return -1L;
}
