#include <ltrace.h>
#include <string.h>

void	striter(char *s, void (*f)(char *))
{
	TRACE
	while (*s)
		f(s++);
}
