#include <custom.h>
#include <errno.h>
#include <stdio.h>

long ftell(FILE *stream)
{
	(void)stream;
	errno = ENOSYS;
	return -1;
}
