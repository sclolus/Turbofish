#include <time.h>
#include <errno.h>
#include <unistd.h>

char *ctime(const time_t *clock)
{
	return asctime(localtime(clock));
}
