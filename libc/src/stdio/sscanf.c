#include <stdio.h>
#include <errno.h>
#include <ltrace.h>

#warning DUMMY IMPLEMENTATION of sscanf.

int      sscanf(const char *restrict s, const char *restrict format, ...)
{
	TRACE
	(void)s;
	(void)format;
	errno = ENOSYS;
	return (int)EOF;
}
