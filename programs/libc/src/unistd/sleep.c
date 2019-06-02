
#include "time.h"

unsigned int sleep(unsigned int seconds) {
	struct timespec input;
	struct timespec output; /* no initialised first, setted by sys_libc */

	input.tv_sec = (time_t)seconds;
	input.tv_nsec = 0;

	int ret = nanosleep(&input, &output);
	/*
	 * RETURN VALUE
	 * Zero if the requested time has elapsed, or the number of seconds
	 * left to sleep, if the call was interrupted by a signal handler
	 */
	if (ret != 0) {
		if (output.tv_sec != 0) {
			return output.tv_sec + output.tv_nsec != 0 ? 1 : 0;
		} else {
			return 1;
		}
	}
	return 0;
}
