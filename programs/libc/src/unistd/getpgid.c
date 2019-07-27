#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

pid_t getpgid(pid_t pid) {
	int ret = _user_syscall(GETPGID, 1, pid);
	if (ret < 0) {
		errno = -ret;
		ret = (pid_t) -1;
	}
	return ret;
}
