#include <ltrace.h>
#include <sys/select.h>
#include <errno.h>

# warning DUMMY IMPLEMENTATION of FD_ISSET
int FD_ISSET(int fd, fd_set *fdset)
{
	TRACE
	(void)fd;
	(void)fdset;
	errno = ENOSYS;
	return -1;
}
