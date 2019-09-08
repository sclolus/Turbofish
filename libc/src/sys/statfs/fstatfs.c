#include <sys/statfs.h>
#include <errno.h>
#include <user_syscall.h>

/// The statfs() system call returns information about a mounted filesystem.
/// `path` is the pathname of any file within the mounted filesystem.
/// `buf` is a  pointer  to  a  statfs structure.
///
/// fstatfs()  returns  the  same information about an
/// open file referenced by descriptor fd.

int fstatfs(int fd, struct statfs *buf)
{
	int ret = _user_syscall(FSTATFS, 2, fd, buf);
	set_errno_and_return(ret);
}
