#include <ltrace.h>
#include <sys/select.h>

# warning DUMMY IMPLEMENTATION of FD_SET
void FD_SET(int fd, fd_set *fdset)
{
	TRACE
	(void)fd;
	(void)fdset;
}
