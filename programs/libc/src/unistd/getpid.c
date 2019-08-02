#include <user_syscall.h>
#include <signal.h>
#include <errno.h>

pid_t getpid(void)
{
	pid_t ret = _user_syscall(GETPID, 0);
	set_errno_and_return(ret);
}
