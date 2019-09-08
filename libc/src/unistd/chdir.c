#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

// The chdir() function shall cause the directory named by the
// pathname pointed to by the path argument to become the current
// working directory; that is, the starting point for path searches
// for pathnames not beginning with '/'.

int chdir(const char *path)
{
	TRACE
	int ret = _user_syscall(CHDIR, 1, path);
	set_errno_and_return(ret);
}
