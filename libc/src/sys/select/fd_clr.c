#include <ltrace.h>
#include <sys/select.h>
#include <custom.h>

# warning DUMMY IMPLEMENTATION of FD_CLR
void FD_CLR(int fd, fd_set *fdset)
{
	TRACE
	DUMMY
	(void)fd;
	(void)fdset;
}
