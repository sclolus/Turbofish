#include <ltrace.h>
#include <time.h>
#include <errno.h>
#include <unistd.h>

char *ctime(const time_t *clock)
{
	TRACE
	return asctime(localtime(clock));
}
