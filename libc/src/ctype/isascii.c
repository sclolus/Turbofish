#include <ltrace.h>
#include <ctype.h>

int isascii(int c)
{
	TRACE
	if (c >> 7)
		return (0);
	return (1);
}
