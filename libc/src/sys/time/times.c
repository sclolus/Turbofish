#include <errno.h>
#include <user_syscall.h>
#include <sys/times.h>

clock_t	times(struct tms *buf)
{
	int ret	= _user_syscall(TIMES, 1, buf);

	set_errno_and_return(ret);
}
