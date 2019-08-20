#include <unistd.h>

// The isatty() function shall test whether fildes, an open file descriptor, is associated with a terminal device.

#warning NOT IMPLEMENTED
#include <custom.h>

int isatty(int fildes)
{
	DUMMY
	(void)fildes;
	return 1;
}
