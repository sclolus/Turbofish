#include <sys/select.h>

# warning DUMMY IMPLEMENTATION of FD_ZERO
void FD_ZERO(fd_set *fdset)
{
	(void)fdset;
}
