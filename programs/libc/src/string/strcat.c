
#include "string.h"

char	*strcat(char *restrict s1, const char *restrict s2)
{
	char *origin;

	origin = s1;
	while (*s1)
		s1++;
	while (*s2)
		*s1++ = *s2++;
	*s1 = '\0';
	return (origin);
}
