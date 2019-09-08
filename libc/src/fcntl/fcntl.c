#include <ltrace.h>
#include <fcntl.h>
#include <errno.h>
#include <user_syscall.h>

#warning "NOT IMPLEMENTED"
#include <ltrace.h>
#include <custom.h>

int fcntl(int fildes, int cmd, ...)
{
	TRACE
	DUMMY
	va_list ap;
	int arg = 0;

	va_start(ap, cmd);
	switch (cmd) {
		case F_DUPFD:
		case F_SETFD:
			arg = va_arg(ap, int);
			break;
	}
	int ret = _user_syscall(FCNTL, 3, fildes, cmd, arg);
	va_end(ap);
	set_errno_and_return(ret);
}
