#include <ltrace.h>
#include <string.h>

void	striteri(char *s, void (*f)(unsigned int, char *))
{
	TRACE
	unsigned int i;

	i = 0;
	while (s[i]) {
		f(i, s + i);
		i++;
	}
}
