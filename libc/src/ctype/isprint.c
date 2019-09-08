#include <ltrace.h>
#include <ctype.h>

int isprint(int c)
{
	TRACE
	if (c >= 32 && c <= 126)
		return (1);
	return (0);
}
