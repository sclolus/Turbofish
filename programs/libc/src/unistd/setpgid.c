#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

int setpgid(pid_t pid, pid_t pgid)
{
	int ret = _user_syscall(SETPGID, 2, pid, pgid);
	set_errno_and_return(ret);
}
