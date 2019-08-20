#include <string.h>

void	bzero(void *s, size_t n)
{
	char *o;

	o = (char *)s;
	while (n--)
		*o++ = 0x00;
}
