#include <ltrace.h>
#include <string.h>

void	bzero(void *s, size_t n)
{
	TRACE
	char *o;

	o = (char *)s;
	while (n--)
		*o++ = 0x00;
}
