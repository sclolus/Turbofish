#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>

extern int errno;

ssize_t write(int fd, const void *s, size_t len)
{
	int ret = _user_syscall(WRITE, 3, fd, s, len);

	set_errno_and_return(ret);
}
