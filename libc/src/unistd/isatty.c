#include <ltrace.h>
#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

// The isatty() function shall test whether fildes, an open file
// descriptor, is associated with a terminal device.
// The isatty() function shall return 1 if fildes is associated with a
// terminal; otherwise, it shall return 0 and may set errno to
// indicate the error.
int isatty(int fildes)
{
	TRACE
	int ret = _user_syscall(ISATTY, 1, fildes);

	if (ret == 0) {
		errno = -ret;
		return 0;
	} else {
		return ret;
	}
}
