#include <sys/resource.h>
#include <errno.h>
#include <ltrace.h>

#warning dummy implementation
int getrusage(int who, struct rusage *usage)
{
	TRACE
	errno = ENOSYS;
	return -1;
}
