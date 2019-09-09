#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

pid_t getpgid(pid_t pid)
{
	TRACE
	pid_t ret = _user_syscall(GETPGID, 1, pid);
	if (ret < 0) {
		errno = -ret;
		return (pid_t) -1;
	}
	else {
		return ret;
	}
}
