#include <ltrace.h>
#include <user_syscall.h>
#include <errno.h>

/*
 * Wait for signal
 */
int pause(void)
{
	TRACE
	int ret = _user_syscall(PAUSE, 0);
	/*
	 * pause() returns only when a signal was caught and the signal-catching function returned.
	 * In this case, pause() returns -1, and errno is set to EINTR.
	 */

	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		// Cannot happened normally
		errno = 0;
		return ret;
	}
}
