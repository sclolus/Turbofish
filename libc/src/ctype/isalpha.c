#include <ltrace.h>
#include <ctype.h>

int isalpha(int c)
{
	TRACE
	if ((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z'))
		return (1);
	return (0);
}
