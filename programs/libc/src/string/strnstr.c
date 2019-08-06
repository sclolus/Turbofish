#include <string.h>

char	*strnstr(const char *big, const char *little, size_t len)
{
	size_t i;

	i = strlen(little);
	if (i == 0)
		return ((char *)big);
	while (*big && len >= i) {
		if (strncmp(big, little, i) == 0)
			return ((char *)big);
		len--;
		big++;
	}
	return (NULL);
}
