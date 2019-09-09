#include <ltrace.h>
#include <stddef.h>
#include <sys/resource.h>
#include <sys/select.h>
#include <wait.h>

#warning RUSAGE IS NOT CORRECLY HANDLED

/*
 * wait3, wait4 - wait for process to change state, BSD style
 */
pid_t wait3(int *wstatus, int options, struct rusage *rusage)
{
	TRACE
	if (rusage != NULL) {
		rusage->ru_utime.tv_sec = 0;
		rusage->ru_utime.tv_usec = 0;
		rusage->ru_stime.tv_sec = 0;
		rusage->ru_stime.tv_usec = 0;
	}
	return waitpid(-1, wstatus, options);
}
