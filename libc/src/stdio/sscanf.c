#include <stdio.h>
#include <errno.h>

#warning DUMMY IMPLEMENTATION of sscanf.

int      sscanf(const char *restrict s, const char *restrict format, ...)
{
	(void)s;
	(void)format;
	errno = ENOSYS;
	return (int)EOF;
}
