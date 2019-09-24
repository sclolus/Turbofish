#include <ltrace.h>
#include <strings.h>
#include <ctype.h>
#include <ltrace.h>

int strncasecmp(const char *s1, const char *s2, size_t n)
{
	TRACE
	const unsigned char *p1 = (const unsigned char *) s1;
	const unsigned char *p2 = (const unsigned char *) s2;
	int	result;
	size_t	count = 0;

	if (p1 == p2)
		return 0;
	while (count < n && (result = _tolower (*p1) - _tolower (*p2++)) == 0) {
		if (*p1++ == '\0')
			break;
		count++;
	}
	return result;
}
