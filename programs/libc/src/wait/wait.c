
#include "wait.h"

extern int user_waitpid(pid_t pid, int *wstatus, int options);
extern int errno;

/*
 * Each of these calls sets errno to an appropriate value in the case of an error
 */

/*
 * wait for process to change state
 */
pid_t waitpid(pid_t pid, int *wstatus, int options)
{
	pid_t p = user_waitpid(pid, wstatus, options);
	/*
	 * on success, returns the process ID of the child whose state
	 * has changed; if WNOHANG was specified and one or more child(ren)
	 * specified by pid exist, but have not yet changed state, then 0
	 * is returned.  On error, -1 is returned.
	 */
	if (p < 0) {
		errno = -p;
		return -1;
	} else {
		errno = 0;
		return p;
	}
}

/*
 * wait for process to change state
 */
pid_t wait(int *wstatus)
{
	/*
	 * on success, returns the process ID of the terminated child;
	 * on error, -1 is returned.
	 */
	return waitpid(-1, wstatus, 0);
}
