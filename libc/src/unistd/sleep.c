#include <time.h>
#include <unistd.h>
#include <errno.h>

/*
 * sleep - sleep for a specified number of seconds
 */
unsigned int sleep(unsigned int seconds)
{
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

/*
 * usleep - suspend execution for microsecond intervals
 */
int usleep(useconds_t usec)
{
	struct timespec input;
	struct timespec output; /* no initialised first, setted by sys_libc */

	#define MICRO 1000000

	input.tv_sec = (time_t)usec / MICRO;
	input.tv_nsec = (usec % MICRO) * 1000;

	int ret = nanosleep(&input, &output);
	/*
	 * RETURN VALUE
	 * The usleep() function returns 0 on success.  On error, -1 is returned,
	 * with errno set to indicate the cause of the error.
	 */
	set_errno_and_return(ret);
}
