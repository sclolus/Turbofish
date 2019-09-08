#include <time.h>
#include <sys/time.h>
#include <custom.h>
#include <errno.h>

time_t time(time_t *tloc)
{
	struct timeval	tv;

	if (-1 == gettimeofday(&tv, NULL)) {
		return (time_t)-1;
	}

	if (tloc) {
		*tloc = tv.tv_sec;
	}
	return tv.tv_sec;
}
