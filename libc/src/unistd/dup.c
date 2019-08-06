#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

int dup(int oldfd)
{
	if (oldfd < 0) {
		errno = EBADF;
		return -1;
	}

	int ret = _user_syscall(DUP, 1, oldfd);

	set_errno_and_return(ret);
}
