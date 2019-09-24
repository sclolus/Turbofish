#include <ltrace.h>
#include <sys/select.h>

# warning DUMMY IMPLEMENTATION of FD_CLR
void FD_CLR(int fd, fd_set *fdset)
{
	TRACE
	(void)fd;
	(void)fdset;
}
