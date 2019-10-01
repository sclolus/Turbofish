#include <ltrace.h>
#include <sys/select.h>
#include <custom.h>

# warning DUMMY IMPLEMENTATION of FD_SET
void FD_SET(int fd, fd_set *fdset)
{
	TRACE
	DUMMY
	(void)fd;
	(void)fdset;
}
