#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

pid_t        getppid(void)
{
	pid_t ret = _user_syscall(GETPPID, 0);
	set_errno_and_return(ret);
}
