#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

// The readlink() function shall place the contents of the symbolic
// link referred to by path in the buffer buf which has size
// bufsize. If the number of bytes in the symbolic link is less than
// bufsize, the contents of the remainder of buf are unspecified. If
// the buf argument is not large enough to contain the link content,
// the first bufsize bytes shall be placed in buf.
// 
// If the value of bufsize is greater than {SSIZE_MAX}, the result is
// implementation-defined.
// 
// Upon successful completion, readlink() shall mark for update the
// last data access timestamp of the symbolic link.

ssize_t readlink(const char *path, char *restrict buf, size_t bufsize)
{
	TRACE

	ssize_t ret = _user_syscall(READLINK, 3, path, buf, bufsize);
	set_errno_and_return(ret);
}
