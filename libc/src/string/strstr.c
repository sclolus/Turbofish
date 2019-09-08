#include <ltrace.h>
#include <string.h>

char	*strstr(const char *big, const char *little)
{
	TRACE
	size_t len;

	if (!(len = strlen(little)))
		return ((char *)big);
	while (*big) {
		if (*big == *little) {
			if (strncmp(big, little, len) == 0)
				return ((char *)big);
		}
		big++;
	}
	return (NULL);
}
