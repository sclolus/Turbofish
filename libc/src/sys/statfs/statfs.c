#include <sys/statfs.h>
#include <errno.h>
#include <user_syscall.h>

/// The statfs() system call returns information about a mounted filesystem.
/// `path` is the pathname of any file within the mounted filesystem.
/// `buf` is a  pointer  to  a  statfs structure.

int statfs(const char *path, struct statfs *buf)
{
	int ret = _user_syscall(STATFS, 2, path, buf);
	set_errno_and_return(ret);
}
