#include <sys/time.h>
#include <stdlib.h>

/// The gettimeofday() function shall obtain the current time,
/// expressed as seconds and microseconds since the Epoch, and store
/// it in the timeval structure pointed to by tp. The resolution of
/// the system clock is unspecified.  If tzp is not a null pointer,
/// the behavior is unspecified.  gettimeofday() function shall return
/// 0 and no value shall be reserved to indicate an error.
#warning DUMMY IMPLEMENTATION

int gettimeofday(struct timeval *restrict tp, void *restrict tzp)
{
	(void)tzp;
	if (tp != NULL) {
		tp->tv_sec = 0;
		tp->tv_usec = 0;
	}
	return 0;
}

