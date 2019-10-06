#include "stdio.h"
#include "libc.h"

int fscanf(FILE *stream, const char *format, ...)
{
	va_list ap;

	va_start(ap, format);
	int n = xvfscanf(stream, format, ap);

	va_end(ap);
	return n;
}
