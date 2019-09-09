#include <ltrace.h>
#include <string.h>

int	memcmp(const void *s1, const void *s2, size_t n)
{
	TRACE
	unsigned char	*s1a;
	unsigned char	*s2a;
	size_t		i;

	s1a = (unsigned char *)s1;
	s2a = (unsigned char *)s2;
	i = 0;
	while (i < n) {
		if (s1a[i] != s2a[i])
			return (s1a[i] - s2a[i]);
		i++;
	}
	return (0);
}

int	ft_memcmp(const void *s1, const void *s2, size_t n)
{
	TRACE
	return memcmp(s1, s2, n);
}
