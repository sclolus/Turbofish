#include "sched.h"
#include "user_syscall.h"

extern int errno;

int	clone(int (*fn)(void *), void *child_stack,
			  int flags, void *arg/*, pid_t *ptid, void *newtls, pid_t *ctid*/) {
int ret = _user_syscall(CLONE, 4, fn, child_stack, flags, arg);

	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return ret;
	}
}
