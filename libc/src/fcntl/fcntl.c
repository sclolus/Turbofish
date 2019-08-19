#include <fcntl.h>
#include <errno.h>

#warning "NOT IMPLEMENTED"
#include <custom.h>

int fcntl(int fildes, int cmd, ...)
{
	DUMMY
	(void)fildes;
	(void)cmd;
	errno = ENOSYS;
	return -1;
}
