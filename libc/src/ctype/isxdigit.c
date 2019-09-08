#include <ltrace.h>
#include <ctype.h>

int isxdigit(int c)
{
	TRACE
	return ((c >= '0' && c <= '9')
		|| (c >= 'a' && c <= 'f')
		|| (c >= 'A' && c <= 'F'));
}
