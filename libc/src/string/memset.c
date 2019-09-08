#include <ltrace.h>
#include <string.h>

void	*memset(void *b, int c, size_t len)
{
	char *s;

	c = (unsigned char)c;
	s = (char *)b;
	while (len--)
		*s++ = c;
	return (b);
}

void	*ft_memset(void *b, int c, size_t len)
{
	return memset(b, c, len);
}
