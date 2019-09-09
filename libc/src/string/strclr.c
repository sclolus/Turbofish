#include <ltrace.h>
#include <string.h>

void	strclr(char *s)
{
	TRACE
	while (*s)
		*s++ = '\0';
}
