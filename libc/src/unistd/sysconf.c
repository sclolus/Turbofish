#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION of sysconf.
long sysconf(int name)
{
	TRACE
	DUMMY
		if (name == _SC_CLK_TCK) {
			return HZ;
		}

	errno = ENOSYS;
	return -1L;
}
