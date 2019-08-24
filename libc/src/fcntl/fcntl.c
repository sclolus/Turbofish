#include <fcntl.h>
#include <errno.h>
#include <user_syscall.h>

#warning "NOT IMPLEMENTED"
#include <custom.h>

int fcntl(int fildes, int cmd, ...)
{

	DUMMY
	va_list ap;
	va_start(ap, cmd);
	int ret = _user_syscall(FCNTL, 3, fildes, cmd, va_arg(ap, int));
	va_end(ap);
	set_errno_and_return(ret);
}
