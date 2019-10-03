#include <sys/mount.h>
#include <ltrace.h>
#include <user_syscall.h>
#include <errno.h>

int mount(const char *source, const char *target,
		  const char *filesystemtype, unsigned long mountflags,
		  const void *data)
{
	TRACE
	int ret = _user_syscall(MOUNT, 5, source, target, filesystemtype, mountflags, data);
	set_errno_and_return(ret);
}
