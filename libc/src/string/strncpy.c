#include <ltrace.h>
#include <string.h>

char	*strncpy(char *dst, const char *src, size_t len)
{
	TRACE
	size_t i;

	i = 0;
	while (i < len) {
		dst[i] = src[i];
		if (src[i] == '\0') {
			i++;
			while (i < len)
				dst[i++] = '\0';
			break ;
		}
		i++;
	}
	return (dst);
}
