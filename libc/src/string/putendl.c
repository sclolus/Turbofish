#include <ltrace.h>
#include <string.h>
#include <stdio.h>

void	putendl(char const *s)
{
	TRACE
	puts(s);
}
