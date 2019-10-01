#include <ltrace.h>
#include <sys/select.h>
#include <custom.h>

# warning DUMMY IMPLEMENTATION of FD_ZERO
void FD_ZERO(fd_set *fdset)
{
	TRACE
	DUMMY
	(void)fdset;
}
