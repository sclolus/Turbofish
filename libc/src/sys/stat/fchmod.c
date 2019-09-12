#include <ltrace.h>
#include <sys/stat.h>
#include <user_syscall.h>
#include <errno.h>

/* The fchmod() function shall be equivalent to chmod() except that
 * the file whose permissions are changed is specified by the file
 * descriptor fildes. */
int fchmod(int fd, mode_t mode)
{
	TRACE
	int ret = _user_syscall(FCHMOD, 2, fd, mode);
	set_errno_and_return(ret);
}
