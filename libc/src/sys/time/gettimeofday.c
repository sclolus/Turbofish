#include <ltrace.h>
#include <sys/time.h>
#include <stdlib.h>
#include <user_syscall.h>
#include <errno.h>

/// The gettimeofday() function shall obtain the current time,
/// expressed as seconds and microseconds since the Epoch, and store
/// it in the timeval structure pointed to by tp. The resolution of
/// the system clock is unspecified.  If tzp is not a null pointer,
/// the behavior is unspecified.  gettimeofday() function shall return
/// 0 and no value shall be reserved to indicate an error.
///
/// However this implementation can fail with EFAULT, if the pointers are invalid.
int gettimeofday(struct timeval *restrict tp, void *restrict tzp)
{
	TRACE
	int ret = _user_syscall(GETTIMEOFDAY, 2, tp, tzp);
	set_errno_and_return(ret);
}
