#include <string.h>
#include <ltrace.h>

char	*stpcpy(char *dst, const char *src)
{
	TRACE
	while (*src)
		*dst++ = *src++;
	*dst = '\0';
	return dst;
}
