#include <ltrace.h>
#include <string.h>

char	*strcpy(char *dst, const char *src)
{
	TRACE
	char *origin;

	origin = dst;
	while (*src)
		*dst++ = *src++;
	*dst = '\0';
	return (origin);
}
