#include "unistd.h"
#include "errno.h"
#include "user_syscall.h"

extern int errno;

pid_t getpgid(pid_t pid) {
	int ret = _user_syscall(GETPGID, 1, pid);
	if (ret < 0) {
		errno = -ret;
		ret = (pid_t) -1;
	}
	return ret;
}

pid_t getpgrp(void) {
	return getpgid(0);
}

int setpgid(pid_t pid, pid_t pgid) {
	int ret = _user_syscall(SETPGID, 2, pid, pgid);
	if (ret < 0) {
		errno = -ret;
		ret = -1;
	}
	return ret;
}
