#include <ltrace.h>
#include <stddef.h>
#include <sys/resource.h>
#include <sys/select.h>
#include <wait.h>

/*
 * wait3, wait4 - wait for process to change state, BSD style
 */
pid_t wait3(int *wstatus, int options, struct rusage *rusage)
{
	TRACE
	/*
	 * on success, returns the process ID of the terminated child;
	 * on error, -1 is returned.
	 */
	return wait4(-1, wstatus, options, rusage);
}
