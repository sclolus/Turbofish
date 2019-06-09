
#include "string.h"

void	striter(char *s, void (*f)(char *))
{
	while (*s)
		f(s++);
}
