#include <stdio.h>
#include <errno.h>
#include <ltrace.h>
#include <libc.h>

#warning DUMMY IMPLEMENTATION of sscanf.

int      sscanf(const char *restrict s, const char *restrict format, ...)
{
	/* TRACE */
	/* (void)s; */
	/* (void)format; */
	/* errno = ENOSYS; */
	/* return (int)EOF; */
	va_list ap;
	/* printf("sscanf('%s', '%s'\n", s, format); */

	va_start(ap, format);
	int n = xvsscanf(s, format, ap);

	va_end(ap);

	return n;
}
