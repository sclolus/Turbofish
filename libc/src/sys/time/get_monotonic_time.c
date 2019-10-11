#include <time.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/time.h>

#warning get_monotonic_time is an experimental function
#include <user_syscall.h>
extern struct kernel kernel;

#define MICRO 1000000

/*
 * Get the monotonic time in micro-seconds since CPU has started
 */
int get_monotonic_time(struct timeval *tv, struct timezone *tz)
{
	(void)tz;

	if (tv == NULL) {
		return -1;
	}
	u32 eax;
	u32 edx;

	__asm("rdtsc" : "={eax}"(eax), "={edx}"(edx));
	uint64_t nb_ticks = ((uint64_t)edx << 32) + (uint64_t)eax;

	uint64_t utime = nb_ticks * MICRO / kernel.cpu_frequency;
	tv->tv_sec = utime / MICRO;
	tv->tv_usec = utime % MICRO;
	return 0;
}
