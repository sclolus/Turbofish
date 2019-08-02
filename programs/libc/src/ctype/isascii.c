#include <ctype.h>

int isascii(int c)
{
	if (c >> 7)
		return (0);
	return (1);
}
