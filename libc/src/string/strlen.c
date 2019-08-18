#include <string.h>

size_t	strlen(const char *s)
{
	const char *t;

	t = s;
	while (*t)
		t++;
	return (t - s);
}
