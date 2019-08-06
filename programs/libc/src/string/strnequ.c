#include <string.h>

int	strnequ(char const *s1, char const *s2, size_t n)
{
	size_t i;

	i = 0;
	while (s1[i] && i < n) {
		if (s1[i] != s2[i])
			return (0);
		i++;
	}
	if (s2[i] == s1[i] || i == n)
		return (1);
	return (0);
}
