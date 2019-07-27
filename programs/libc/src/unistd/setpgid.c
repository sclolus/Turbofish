#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

int setpgid(pid_t pid, pid_t pgid) {
	int ret = _user_syscall(SETPGID, 2, pid, pgid);
	if (ret < 0) {
		errno = -ret;
		ret = -1;
	}
	return ret;
}
