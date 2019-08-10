#include <fcntl.h>
#include <errno.h>

#warning "NOT IMPLEMENTED"

int fcntl(int fildes, int cmd, ...)
{
	errno = ENOSYS;
	return -1;
}
