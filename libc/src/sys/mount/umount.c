#include <sys/mount.h>
#include <ltrace.h>
#include <user_syscall.h>
#include <errno.h>


int umount(const char *target)
{
	TRACE
	int ret = _user_syscall(UMOUNT, 1, target);
	set_errno_and_return(ret);
}
