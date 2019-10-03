#include <sched.h>
#include <errno.h>
#include <ltrace.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION

int sched_setscheduler(pid_t pid, int policy,
		       const struct sched_param *param)
{
	TRACE
	DUMMY
	(void)pid;
	(void)policy;
	(void)param;
	errno = ENOSYS;
	return -1;
}
