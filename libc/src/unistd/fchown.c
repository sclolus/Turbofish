#include <ltrace.h>
#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/* The fchown() function shall be equivalent to chown() except that
 * the file whose owner and group are changed is specified by the file
 * descriptor fildes. */
int          fchown(int fd, uid_t owner, gid_t group)
{
	TRACE
	int ret = _user_syscall(FCHOWN, 3, fd, owner, group);
	set_errno_and_return(ret);

}
