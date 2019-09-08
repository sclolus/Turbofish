#include <ltrace.h>
#include <user_syscall.h>
#include <fcntl.h>
#include <errno.h>

static int vaarg_open(const char *path, int oflag, va_list ap)
{
	TRACE
	int arg;

	// Get new file stats if found a new file
	if (oflag & O_CREAT) {
		arg = va_arg(ap, int);
	} else {
		arg = 0;
	}

	int ret = _user_syscall(OPEN, 3, path, oflag, arg);
	/*
	 * open() return the new file descriptor, or -1 if an error
	 * occurred (in which case, errno is set appropriately)
	 */
	set_errno_and_return(ret);
}

/*
 * open and possibly create a file
 */
int open(const char *path, int oflag, ...)
{
	TRACE
	va_list ap;
	va_start(ap, oflag);
	int n = vaarg_open(path, oflag, ap);
	va_end(ap);
	return n;
}

int open64(const char *path, int oflag, ...)
{
	TRACE
	va_list ap;
	va_start(ap, oflag);
	int n = vaarg_open(path, oflag, ap);
	va_end(ap);
	return n;
}
