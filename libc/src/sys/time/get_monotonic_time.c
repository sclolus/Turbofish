#include <time.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/time.h>

#warning get_monotonic_time is a custom function

int  get_monotonic_time(struct timeval *tv, struct timezone *tz)
{
	(void)tz;
	if (tv == NULL) {
		return -1;
	}
	u32 eax;
	u32 edx;

	__asm("rdtsc" : "={eax}"(eax), "={edx}"(edx));
	tv->tv_sec = edx;
	tv->tv_usec = eax;
	return 0;
}
