#include <sys/select.h>
#include <errno.h>

# warning DUMMY IMPLEMENTATION of FD_ISSET
int FD_ISSET(int fd, fd_set *fdset)
{
	(void)fd;
	(void)fdset;
	errno = ENOSYS;
	return -1;
}
