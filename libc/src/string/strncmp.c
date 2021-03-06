#include <ltrace.h>
#include <string.h>

int	strncmp(const char *s1, const char *s2, size_t n)
{
	TRACE
	size_t i;

	i = 0;
	while (i < n) {
		if ((s1[i] != s2[i]) || (s1[i] == '\0'))
			return ((unsigned char)s1[i] - (unsigned char)s2[i]);
		i++;
	}
	return (0);
}
