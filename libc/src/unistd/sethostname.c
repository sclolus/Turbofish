#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

int          sethostname(const char *name, size_t namelen) // shall be size_t by posix.
{
	int ret = _user_syscall(SETHOSTNAME, 2, name, namelen);

	set_errno_and_return(ret);
}
