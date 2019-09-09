#include <ltrace.h>
#include <string.h>

char	*strchr(const char *s, int c)
{
	TRACE
	while (*s) {
		if (*s == c)
			return ((char *)s);
		s++;
	}
	if (c == '\0' && *s == '\0')
		return ((char *)s);
	return (NULL);
}
