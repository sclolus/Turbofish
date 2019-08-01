
#include <string.h>

char	*strrchr(const char *s, int c)
{
	char *tmp;

	tmp = NULL;
	while (*s) {
		if (*s == c)
			tmp = (char *)s;
		s++;
	}
	if (c == '\0' && *s == '\0')
		return ((char *)s);
	if (tmp != NULL)
		return (tmp);
	return (NULL);
}
